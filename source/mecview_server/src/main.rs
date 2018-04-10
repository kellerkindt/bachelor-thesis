
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
use std::io::BufRead;
use std::net::SocketAddr;

use async::Sink;
use async::Future;
use async::Stream;
use async::AsyncRead;
use io::net::TcpStream;

fn main() {

    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    let address = "0.0.0.0:5500".parse::<SocketAddr>().unwrap();

    info!("Staring Server v{} on interface {}", env!("CARGO_PKG_VERSION"), address);
    let server = server::Server::new(address).unwrap();
    let server = server.start().unwrap();

    info!("Server started successfully");

    let stdin = ::std::io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Err(_) => break,
            Ok(line) => match line.trim() {
                _ if line.is_empty() => {},
                "quit"|"q"|"exit" => break,
                _ => println!(":: Unknown command")
            }
        }
    }
}
