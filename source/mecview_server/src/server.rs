
use std::io::Error;
use std::io::ErrorKind;
use std::fmt::Debug;
use std::net::SocketAddr;

use client;
use client::Client;

use adapter;
use adapter::asn::AsnClientAdapter;

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

use io::net::TcpStream;
use io::net::TcpListener;

use libmessages::Message;


const CHANNEL_BUFFER_SIZE_SERVER  : usize = 10;
const CHANNEL_BUFFER_SIZE_CLIENT  : usize = 10;
const CHANNEL_BUFFER_SIZE_ADAPTER : usize = 10;

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
        info!("Client connected from {}", client.peer_addr().unwrap());

        if client.set_nodelay(true).is_err() {
            warn!("TCP nodelay couldn't be set");
        }
        if client.set_keepalive(Some(::std::time::Duration::from_millis(1_000))).is_err() {
            warn!("TCP keepalive couldn't be set");
        }

        let (adapter_tx, adapter_rx) : (Sender<adapter::Command<Message>>, _) = channel(CHANNEL_BUFFER_SIZE_ADAPTER);
        let client_tx  = self.spawn_new_client(adapter_tx.clone());

        let (encoder, decoder) = client.framed(adapter::asn::AsnCodec()).split();

        self.spawn_new_asn_adapter(encoder, adapter_rx, client_tx);
        self.spawn_decoder_adapter_adapter(decoder, adapter_tx);
    }

    /// Spawns a new `Client` that listens for commands on the returned `async::Sender`
    /// and uses the given `async::Sender` to send messages to the underlying remote client
    fn spawn_new_client<M: Debug+Send+'static>(&mut self, adapter: Sender<adapter::Command<M>>) -> Sender<client::Command> {
        let (sender, receiver) = channel(CHANNEL_BUFFER_SIZE_CLIENT);
        let mut client = Client::new(adapter);
        self.runtime.spawn(receiver.for_each(move |command| {
            match client.process_command(command) {
                Ok(()) => Ok(()),
                Err(e) => {
                    error!("Error while processing client command: {:?}", e);
                    Err(())
                },
            }
        }));
        sender
    }


    fn spawn_new_asn_adapter<E: Sink<SinkItem=Message,SinkError=Error> + Send + 'static>(&mut self, encoder: E, commands: Receiver<adapter::Command<Message>>, client: Sender<client::Command>) {
        let mut adapter = AsnClientAdapter::new(encoder, client);
        self.runtime.spawn(commands.for_each(move |command| {
            match adapter.process_command(command) {
                Ok(()) => Ok(()),
                Err(e) => {
                    error!("Error while processing adapter command: {:?}", e);
                    Err(())
                },
            }
        }));
    }

    fn spawn_decoder_adapter_adapter<M: Debug+Send+'static, S: Stream<Item=M,Error=Error>+Send+'static>(&mut self, encoder: S, adapter: Sender<adapter::Command<M>>) {
        let mut adapter = adapter.wait();
        self.runtime.spawn(encoder.for_each(move |message| {
            match adapter.send(adapter::Command::ProcessMessage(message)) {
                Ok(()) => Ok(()),
                Err(e) => {
                    error!("Error while enqueuing encoder message into adapter: {:?}", e);
                    Err(Error::from(ErrorKind::UnexpectedEof))
                },
            }
        }).then(|r| match r {
            Err(_) => Err(()),
            Ok(_) => Ok(()),
        }));
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