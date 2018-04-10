
#[macro_use]
extern crate log;
extern crate log4rs;

extern crate tokio;
extern crate tokio_io;
extern crate futures;

extern crate bytes;
extern crate byteorder;

extern crate libmessages as messages;

mod client;
mod server;
mod adapter;
mod algorithm;
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

fn main() {

    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let server = server::Server::new("0.0.0.0:5500".parse::<SocketAddr>().unwrap()).unwrap();
    let server = server.start().unwrap();

    {
        use std::sync::Arc;
        use messages::asn::AsnMessage;
        use messages::asn::Generalize;

        thread::sleep(::std::time::Duration::from_millis(1_000));

        let stream = TcpStream::connect(&"0.0.0.0:5500".parse::<SocketAddr>().unwrap()).wait().unwrap();
        let (sink, stream) = stream.framed(::server::RawMessageCodec::default()).split();
        let reg = messages::asn::raw::ClientRegistration::decode_from_buffer(&[0x20]).unwrap();
        let arc = Arc::new(reg.encode().unwrap().generalize());
        let sink = sink.send(arc.clone()).wait().unwrap();
        let sink = sink.send(arc).wait().unwrap();
        println!(":::::::::: {:?}", stream.wait().next());
    }

    let time_wait = 100;
    for i in 0..time_wait {
        println!(" === Shutdown in {}s", time_wait-i);
        thread::sleep(::std::time::Duration::from_millis(1_000));
    }
}
