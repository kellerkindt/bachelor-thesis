
use std::io::Error;
use std::io::ErrorKind;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;

use client;
use client::Client;

use adapter;
use adapter::asn::AsnAdapter;

use async::Sink;
use async::spawn;
use async::Future;
use async::Sender;
use async::Stream;
use async::Runtime;
use async::channel;
use async::Receiver;
use async::AsyncRead;
use async::AsyncWrite;
use async::CommandProcessor;

use io::Encoder;
use io::Decoder;
use io::net::TcpStream;
use io::net::TcpListener;

use messages;
use messages::RawMessage;

use bytes::BufMut;
use bytes::BytesMut;
use byteorder::ByteOrder;
use byteorder::NetworkEndian;

const CHANNEL_BUFFER_SIZE_SERVER  : usize = 10;
const CHANNEL_BUFFER_SIZE_CLIENT  : usize = 10;
const CHANNEL_BUFFER_SIZE_ADAPTER : usize = 10;

const HEADER_SIZE : usize = 8;


pub struct Server {
    address: SocketAddr,
    runtime: Runtime,
}

impl Server {
    pub fn new(address: SocketAddr) -> Result<Server, Error> {
        Ok(Server {
            address,
            runtime: Runtime::new()?
        })
    }

    pub fn start(mut self) -> Result<Sender<Command>, Error> {
        let (sender, receiver) = channel(CHANNEL_BUFFER_SIZE_SERVER);
        self.spawn_tcp_listener(sender.clone())?;
        self.spawn_command_processor(receiver);
        Ok(sender)
    }

    fn spawn_tcp_listener(&mut self, sender: Sender<Command>) -> Result<(), Error> {
        let mut sender = sender.wait();
        self.runtime.spawn(
            TcpListener::bind(&self.address)?.incoming().for_each(move |stream| {
                match sender.send(Command::AcceptStream(stream)) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Error::from(ErrorKind::UnexpectedEof)),
                }
            }).then(|r| match r {
                Err(_) => Err(()),
                Ok(_) => Ok(()),
            })
        );
        Ok(())
    }

    fn handle_new_client(&mut self, client: TcpStream) {
        let address = client.peer_addr().unwrap();
        info!("Client connected from {}", address);

        if client.set_nodelay(true).is_err() {
            warn!("TCP nodelay couldn't be set");
        }

        if client.set_keepalive(Some(::std::time::Duration::from_millis(1_000))).is_err() {
            warn!("TCP keepalive couldn't be set");
        }

        // TODO
        let (algorithm_tx, algorithm_rx) = channel(2);
        let (client_tx, client_rx) = channel(2);

        let (encoder, decoder) = client.framed(RawMessageCodec::default()).split();
        let mut client = Client::new(client_tx.clone(), address, AsnAdapter::new(encoder), algorithm_tx.clone());


        self.runtime.spawn(
            decoder
                .map(|m| ::messages::asn::Message::decode(&m))
                .then(|r| match r {
                    Ok(message) => match message {
                        Ok(message) => ::adapter::asn::map_message(message),
                        Err(_) => Err(::std::io::Error::from(::std::io::ErrorKind::UnexpectedEof)),
                    },
                    Err(_) => Err(::std::io::Error::from(::std::io::ErrorKind::UnexpectedEof)),
                })
                .then(|r| match r {
                    Ok(command) => Ok(command),
                    Err(e) => {
                        error!("AsnAdapter error: {:?}", e);
                        // TODO verify
                        Err(::std::io::Error::from(::std::io::ErrorKind::UnexpectedEof))
                    }
                })
                .select(client_rx.then(|r| match r {
                    Ok(ok) => Ok(ok),
                    Err(_) => Err(::std::io::Error::from(::std::io::ErrorKind::UnexpectedEof))

                }))
                .for_each(move |command| client.process_command(command))
                .then(|r| match r {
                    Ok(_) => Ok(()),
                    Err(_) => Err(())
                })
        );
    }

    fn spawn_command_processor(mut self, receiver: Receiver<Command>) {
        let executor = self.runtime.executor().clone();
        executor.spawn(receiver.for_each(move |command| {
            match self.process_command(command) {
                Err(_) => Err(()),
                Ok(_) => Ok(()),
            }
        }));
    }
}


impl CommandProcessor<Command> for Server {
    fn process_command(&mut self, command: Command) -> Result<(), Error> {
        match command {
            Command::AcceptStream(stream) => self.handle_new_client(stream),
        };
        Ok(())
    }
}


pub enum Command {
    AcceptStream(TcpStream),
}

pub struct RawMessageCodec<T> {
    _t: ::std::marker::PhantomData<T>,
}

impl<T> Default for RawMessageCodec<T> {
    fn default() -> RawMessageCodec<T> {
        RawMessageCodec {
            _t: ::std::marker::PhantomData,
        }
    }
}

impl<T> Encoder for RawMessageCodec<T> {
    type Item = Arc<RawMessage<T>>;
    type Error = Error;

    fn encode(&mut self, item: <Self as Encoder>::Item, dst: &mut BytesMut) -> Result<(), <Self as Encoder>::Error> {
        trace!("Writing RawMessage, identifier={}, length={}", item.identifier(), item.length());
        dst.reserve(HEADER_SIZE + item.length() as usize);
        dst.put_u32::<NetworkEndian>(item.length());
        dst.put_u32::<NetworkEndian>(item.identifier());
        dst.put_slice(item.bytes());
        trace!("RawMessage written successfully");
        Ok(())
    }
}

impl<T> Decoder for RawMessageCodec<T> {
    type Item = Arc<RawMessage<T>>;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<<Self as Decoder>::Item>, <Self as Decoder>::Error> {
        trace!("Trying to read RawMessage, available bytes: {}", src.len());
        if src.len() >= HEADER_SIZE {

            let length       = NetworkEndian::read_u32(&src[0..4]) as usize;
            let identifier   = NetworkEndian::read_u32(&src[4..8]);
            let total_length = HEADER_SIZE + length;

            if src.len() >= total_length {
                trace!("Going to read RawMessage");
                let mut vec = vec![0u8; length];
                let mut src = src.split_to(total_length);
                vec[..length].clone_from_slice(&src[HEADER_SIZE..]);
                match RawMessage::new(identifier, vec) {
                    Ok(message) => Ok(Some(Arc::new(message))),
                    Err(_) => Err(Error::from(ErrorKind::InvalidData))
                }

            } else {
                let capacity = src.capacity();
                src.reserve(total_length - capacity);
                Ok(None)
            }
        } else {
            src.reserve(HEADER_SIZE);
            Ok(None)
        }
    }
}