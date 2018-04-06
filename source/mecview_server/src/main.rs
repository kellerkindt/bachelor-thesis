
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
mod server;
mod adapter;
mod command_processor;

mod async {
    pub use futures::Future;
    pub use futures::Stream;
    pub use futures::Sink;
    pub use futures::sink;
    pub use futures::Async;
    pub use futures::sync::mpsc::Sender;
    pub use futures::sync::mpsc::SendError;
    pub use futures::sync::mpsc::Receiver;
    pub use futures::sync::mpsc::channel;

    pub use tokio_io::AsyncRead;
    pub use tokio_io::AsyncWrite;

    pub use tokio::spawn;
    pub use tokio::runtime::Runtime;

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

use std::thread;
use std::net::SocketAddr;

use async::Sink;
use async::Future;
use async::Stream;
use async::AsyncRead;
use io::net::TcpStream;

use libmessages::Message;

fn main() {

    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let server = server::Server::new("0.0.0.0:5500".parse::<SocketAddr>().unwrap()).unwrap();
    let server = server.start();

    thread::sleep_ms(1_000);
    let stream = TcpStream::connect(&"0.0.0.0:5500".parse::<SocketAddr>().unwrap()).wait().unwrap();
    let (sink, stream) = stream.framed(::adapter::asn::AsnCodec()).split();
    let sink = sink.send(Message::decode_client_registration(&[0x20]).unwrap()).wait().unwrap();
    let sink = sink.send(Message::decode_client_registration(&[0x20]).unwrap()).wait().unwrap();

    let time_wait = 100;
    for i in 0..time_wait {
        println!(" === Shutdown in {}s", time_wait-i);
        thread::sleep_ms(1_000);
    }
}
