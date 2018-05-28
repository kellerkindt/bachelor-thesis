use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use client;
use client::Client;

use adapter::asn::AsnAdapter;
use algorithm;
use algorithm::AlgorithmManager;

use async::channel;
use async::AsyncRead;
use async::CommandProcessor;
use async::Future;
use async::Receiver;
use async::Runtime;
use async::Sender;
use async::Sink;
use async::Stream;

use io::net::TcpListener;
use io::net::TcpStream;
use io::Decoder;
use io::Encoder;

use messages::asn;
use messages::asn::raw;
use messages::asn::AsnMessage;
use messages::asn::Generalize;
use messages::RawMessage;

use byteorder::ByteOrder;
use byteorder::NetworkEndian;
use bytes::BufMut;
use bytes::BytesMut;
use libalgorithm_sys::ExternalAlgorithm;

const CHANNEL_BUFFER_SIZE_SERVER: usize = 10;
const CHANNEL_BUFFER_SIZE_CLIENT: usize = 64;
const CHANNEL_BUFFER_SIZE_CLIENT_ALGORITHM: usize = 32;
const CHANNEL_BUFFER_SIZE_ALGORITHM: usize = 128;

const HEADER_SIZE: usize = 8;

type Clt = Sender<client::Command<raw::SensorFrame, raw::EnvironmentFrame>>;
type Alg = Sender<algorithm::Command<raw::SensorFrame, raw::EnvironmentFrame, SocketAddr>>;

pub struct Server {
    address: SocketAddr,
    runtime: Runtime,
    algorithm: Option<Alg>,
    init_message: Option<Arc<RawMessage<asn::Message>>>,
    environment_frame: Option<Box<raw::EnvironmentFrame>>,
    clients: Vec<Clt>,
    algorithm_config: String,
}

impl Server {
    pub fn new(address: SocketAddr, algorithm_config: String) -> Result<Server, Error> {
        Ok(Server {
            address,
            runtime: Runtime::new()?,
            algorithm: None,
            init_message: None,
            environment_frame: None,
            clients: Vec::with_capacity(128),
            algorithm_config,
        })
    }

    // TODO this does not fit in here very well
    pub fn load_init_message<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Error> {
        let mut xml = String::new();
        let _ = File::open(path)?.read_to_string(&mut xml)?;
        match <raw::InitMessage as asn::AsnMessage>::try_decode_xer(&xml) {
            Err(_) => Err(Error::from(ErrorKind::InvalidData)),
            Ok(init) => {
                self.init_message = Some(Arc::new(
                    init.try_encode_uper()
                        .map_err(|_| Error::from(ErrorKind::InvalidData))?
                        .generalize(),
                ));
                Ok(())
            }
        }
    }

    // TODO this does not fit in here very well
    pub fn load_environment_frame<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Error> {
        let mut xml = String::new();
        let _ = File::open(path)?.read_to_string(&mut xml)?;
        match <raw::EnvironmentFrame as asn::AsnMessage>::try_decode_xer(&xml) {
            Err(_) => Err(Error::from(ErrorKind::InvalidData)),
            Ok(init) => {
                self.environment_frame = Some(init);
                Ok(())
            }
        }
    }

    pub fn start(mut self) -> Result<Sender<Command>, Error> {
        let (sender, receiver) = channel(CHANNEL_BUFFER_SIZE_SERVER);
        self.spawn_tcp_listener(sender.clone())?;
        self.spawn_algorithm()?;
        self.spawn_command_processor(receiver);
        Ok(sender)
    }

    fn spawn_tcp_listener(&mut self, sender: Sender<Command>) -> Result<(), Error> {
        let mut sender = sender.wait();
        self.runtime.spawn(
            TcpListener::bind(&self.address)?
                .incoming()
                .for_each(
                    move |stream| match sender.send(Command::AcceptStream(stream)) {
                        Ok(_) => Ok(()),
                        Err(_) => Err(Error::from(ErrorKind::UnexpectedEof)),
                    },
                )
                .then(|r| match r {
                    Err(_) => Err(()),
                    Ok(_) => Ok(()),
                }),
        );
        Ok(())
    }

    fn spawn_algorithm(&mut self) -> Result<(), Error> {
        let (tx, rx) = channel(CHANNEL_BUFFER_SIZE_ALGORITHM);

        let mut alg = unsafe {
            let mut tx = tx.clone().wait();
            ExternalAlgorithm::new(
                &self.algorithm_config,
                move |frame: &::messages::asn::raw::EnvironmentFrame| {
                    trace!("EnvironmentFrame from ExternalAlgorithm received");
                    match frame.try_encode_uper() {
                        Err(e) => error!("Failed to encode EnvironmentFrame: {:?}", e),
                        Ok(encoded) => {
                            trace!("EnvironmentFrame from ExternalAlgorithm encoded successfully");
                            let r = tx.send(::algorithm::Command::Publish(encoded));
                            trace!("sending result {:?}", r.is_ok());
                            if let Err(e) = r {
                                trace!("sending error {:?}", e);
                            }
                        }
                    }
                },
                move |frame: &::messages::asn::raw::InitMessage| {
                    println!("Algorithm InitMessage: {:?}", frame);
                },
            ).expect("Creating ExternalAlgorithm failed")
        };

        let mut sample =
            AlgorithmManager::new(move |frame: Box<::messages::asn::raw::SensorFrame>| {
                alg.send_sensor_frame(frame);
            });

        sample.set_environment_frame(self.environment_frame.take());
        self.runtime.spawn(
            rx.for_each(move |command| match sample.process_command(command) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }),
        );
        self.algorithm = Some(tx);
        Ok(())
    }

    fn spawn_or_get_algorithm(&mut self) -> Result<&mut Alg, Error> {
        if let Some(ref mut alg) = self.algorithm {
            Ok(alg)
        } else {
            self.spawn_algorithm()?;
            match self.algorithm {
                Some(ref mut alg) => Ok(alg),
                None => Err(Error::from(ErrorKind::Other)),
            }
        }
    }

    fn new_client_to_algorithm_buffered_channel(&mut self) -> Result<Alg, Error> {
        let (tx, rx) = channel(CHANNEL_BUFFER_SIZE_CLIENT_ALGORITHM);
        let mut alg = self.spawn_or_get_algorithm()?.clone().wait();
        self.runtime
            .spawn(rx.for_each(move |v| alg.send(v).map_err(|_| ())));
        Ok(tx)
    }

    fn handle_new_client(&mut self, client: TcpStream) -> Result<(), Error> {
        let (client_tx, client_rx) = channel(CHANNEL_BUFFER_SIZE_CLIENT);

        let address = client.peer_addr()?;
        let id = self.next_client_id(client_tx.clone());

        info!("Client connected from {}, assigned id {}", address, id);

        if client.set_nodelay(true).is_err() {
            warn!("TCP nodelay couldn't be set");
        }

        if client
            .set_keepalive(Some(Duration::from_millis(1_000)))
            .is_err()
        {
            warn!("TCP keepalive couldn't be set");
        }

        let (encoder, decoder) = client.framed(RawMessageCodec::default()).split();
        let mut client = Client::new(
            client_tx,
            address,
            AsnAdapter::new(encoder, self.init_message.clone()),
            self.new_client_to_algorithm_buffered_channel()?,
        );

        self.runtime.spawn(
            decoder
                .map(|m| ::messages::asn::Message::try_decode_uper(&m))
                .then(move |r| match r {
                    Ok(message) => match message {
                        Ok(message) => ::adapter::asn::map_message(id, message),
                        Err(_) => Err(::std::io::Error::from(::std::io::ErrorKind::UnexpectedEof)),
                    },
                    Err(_) => Err(::std::io::Error::from(::std::io::ErrorKind::UnexpectedEof)),
                })
                // stop the stream by inserting EOF after the decoder is done
                .chain(::futures::stream::once(Err(::std::io::Error::from(::std::io::ErrorKind::UnexpectedEof))))
                .select(client_rx.then(|r| match r {
                    Ok(ok) => Ok(ok),
                    Err(_) => Err(::std::io::Error::from(::std::io::ErrorKind::UnexpectedEof))

                }))
                .for_each(move |command| client.process_command(command))
                .then(|r| match r {
                    Ok(_) => Ok(()),
                    Err(_) => Err(())
                }),
        );

        Ok(())
    }

    fn next_client_id(&mut self, client: Clt) -> usize {
        for id in 0..self.clients.len() {
            if !self.is_client_alive(id) {
                self.clients[id] = client;
                return id;
            }
        }
        let id = self.clients.len() as usize;
        self.clients.push(client);
        id
    }

    fn is_client_alive(&mut self, id: usize) -> bool {
        self.clients
            .get_mut(id)
            .and_then(|ref mut channel| Some(!channel.poll_ready().is_err()))
            .unwrap_or(false)
    }

    fn spawn_command_processor(mut self, receiver: Receiver<Command>) {
        let executor = self.runtime.executor().clone();
        executor.spawn(
            receiver.for_each(move |command| match self.process_command(command) {
                Err(_) => Err(()),
                Ok(_) => Ok(()),
            }),
        );
    }
}

impl CommandProcessor<Command> for Server {
    fn process_command(&mut self, command: Command) -> Result<(), Error> {
        match command {
            Command::AcceptStream(stream) => self.handle_new_client(stream),
        }
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

    fn encode(
        &mut self,
        item: <Self as Encoder>::Item,
        dst: &mut BytesMut,
    ) -> Result<(), <Self as Encoder>::Error> {
        trace!(
            "Writing RawMessage, identifier={}, length={}",
            item.identifier(),
            item.length()
        );
        let mut header = [0u8; HEADER_SIZE];
        NetworkEndian::write_u32(&mut header[0..], item.length());
        NetworkEndian::write_u32(&mut header[4..], item.identifier());

        dst.reserve(HEADER_SIZE + item.length() as usize);
        dst.put_slice(&header);
        dst.put_slice(item.bytes());
        trace!(
            "RawMessage written successfully. Header: {:?}, Body: {:?}",
            header,
            item.bytes()
        );
        Ok(())
    }
}

impl<T> Decoder for RawMessageCodec<T> {
    type Item = Arc<RawMessage<T>>;
    type Error = Error;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<<Self as Decoder>::Item>, <Self as Decoder>::Error> {
        trace!("Trying to read RawMessage, available bytes: {}", src.len());
        if src.len() >= HEADER_SIZE {
            let length = NetworkEndian::read_u32(&src[0..4]) as usize;
            let identifier = NetworkEndian::read_u32(&src[4..8]);
            let total_length = HEADER_SIZE + length;

            if src.len() >= total_length {
                trace!(
                    "Going to read RawMessage, length={}, identifier={}, src={:?}",
                    length,
                    identifier,
                    &src[..total_length]
                );
                let mut message = src.split_to(total_length);
                trace!(
                    "Complete message len={} {:?}",
                    message.len(),
                    &message[..message.len()]
                );
                let _header = message.split_to(HEADER_SIZE);
                trace!(
                    "Message body len={} {:?}",
                    message.len(),
                    &message[..message.len()]
                );
                Ok(Some(Arc::new(RawMessage::new(identifier, message)
                    .map_err(|_| Error::from(ErrorKind::InvalidData))?)))
            } else {
                let capacity = src.capacity();
                if capacity < total_length {
                    src.reserve(total_length - capacity);
                }
                Ok(None)
            }
        } else {
            src.reserve(HEADER_SIZE);
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder() {
        let msg: RawMessage<()> = RawMessage::new(0xFF, vec![0x01, 0xFF, 0x90]).unwrap();
        let mut codec = RawMessageCodec::default();
        let mut bytes = BytesMut::new();
        codec
            .encode(Arc::new(msg), &mut bytes)
            .expect("Encoding failed");
        assert_eq!(
            &[0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0xFF, 0x01, 0xFF, 0x90,],
            &bytes[..11],
        )
    }

    #[test]
    fn test_decoder() {
        let mut codec: RawMessageCodec<()> = RawMessageCodec::default();
        let mut bytes = BytesMut::new();
        bytes.reserve(2);
        bytes.put_slice(&[0x00, 0x00]);
        assert!(
            codec
                .decode(&mut bytes)
                .expect("Erroneous for missing bytes")
                .is_none()
        );
        bytes.reserve(2);
        bytes.put_slice(&[0x00, 0x03]);
        assert!(
            codec
                .decode(&mut bytes)
                .expect("Erroneous for missing bytes")
                .is_none()
        );
        bytes.reserve(4);
        bytes.put_slice(&[0x00_u8, 0x00, 0x00, 0xFF]);
        assert!(
            codec
                .decode(&mut bytes)
                .expect("Erroneous for missing bytes")
                .is_none()
        );
        bytes.reserve(3);
        bytes.put_slice(&[0x01, 0xFF, 0x90]);
        let msg = codec
            .decode(&mut bytes)
            .expect("Decoder erroneous")
            .expect("No message decoded");
        assert_eq!(0xFF, msg.identifier());
        assert_eq!(3, msg.bytes().len());
        assert_eq!(&[0x01, 0xFF, 0x90], &msg.bytes()[..]);
    }
}
