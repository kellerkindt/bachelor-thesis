
#[macro_use]
extern crate log;
extern crate log4rs;

extern crate tokio;
extern crate tokio_io;

#[macro_use]
extern crate futures;

extern crate bytes;
extern crate byteorder;

extern crate libmessages;

mod client;
mod adapter;
mod command_processor;

mod async {
    pub use futures::Future;
    pub use futures::Stream;
    pub use futures::Sink;
    pub use futures::Async;
    pub use futures::sync::mpsc::Sender;
    pub use futures::sync::mpsc::SendError;
    pub use futures::sync::mpsc::Receiver;
    pub use futures::sync::mpsc::channel;

    pub use tokio::spawn;

    pub use command_processor::CommandProcessor;
}

mod io {
    pub use tokio::io::Error;
    pub use tokio::io::AsyncRead;
    pub use tokio::io::AsyncWrite;

    pub use tokio_io::codec::Encoder;
    pub use tokio_io::codec::Decoder;

    pub mod net {
        pub use tokio::net::TcpStream;
        pub use tokio::net::TcpListener;
    }
}

fn main() {
    use std::net::SocketAddr;
    use std::net::SocketAddrV4;
    use async::Sink;
    use async::Future;
    use async::Stream;

    use io::AsyncRead;
    use io::AsyncWrite;
    use io::net::TcpStream;
    use io::net::TcpListener;

    use libmessages::Message;

    use std::thread;



    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();

    let listener : TcpListener = TcpListener::bind(&"0.0.0.0:5500".parse::<SocketAddr>().unwrap()).unwrap();

    runtime.spawn(listener.incoming().for_each(move |stream : TcpStream| {
        println!("new connection");
        let (sink, stream) = stream.framed(::adapter::asn::AsnCodec()).split();
        ::tokio::spawn(stream.for_each(|message| {
            println!("New message: {:?}", message);
            Ok(())
        }).then(|r| match r {
            Err(_) => Err(()),
            Ok(()) => Ok(()),
        }));

        let message : Message = Message::decode_client_registration(&[0x20, 0x00, 0x00]).unwrap();
        sink.send(message).wait().unwrap();

        println!("new connection end");
        Ok(())
    }).then(|r| match r {
        Err(_) => Err(()),
        Ok(()) => Ok(()),
    }));

    thread::sleep_ms(1_000);
    let stream = TcpStream::connect(&"0.0.0.0:5500".parse::<SocketAddr>().unwrap()).wait().unwrap();
    let (sink, stream) = stream.framed(::adapter::asn::AsnCodec()).split();
    sink.send(Message::decode_client_registration(&[0x20, 0x00, 0x00]).unwrap()).wait();
    thread::sleep_ms(10_000);
}


use async::Stream;
use async::CommandProcessor;

const CHANNEL_BUFFER_SIZE_CLIENT: usize = 2;

fn spawn_new_client(adapter: async::Sender<adapter::Command<libmessages::Message>>) -> async::Sender<client::Command> {
    let (sender, receiver) = async::channel(CHANNEL_BUFFER_SIZE_CLIENT);
    let mut client = client::Client::new(adapter);
    async::spawn(receiver.for_each(move |command : client::Command| {
        client.process_command(command)
    }));
    sender
}