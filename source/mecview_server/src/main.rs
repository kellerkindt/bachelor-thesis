#[macro_use]
extern crate log;
extern crate log4rs;

extern crate futures;
extern crate tokio;
extern crate tokio_io;

extern crate byteorder;
extern crate bytes;

extern crate libmessages as messages;

mod adapter;
mod algorithm;
mod client;
mod command_processor;
mod server;

mod async {
    pub use futures::sink;
    pub use futures::sync::mpsc::channel;
    pub use futures::sync::mpsc::Receiver;
    pub use futures::sync::mpsc::SendError;
    pub use futures::sync::mpsc::Sender;
    pub use futures::Async;
    pub use futures::Future;
    pub use futures::Sink;
    pub use futures::Stream;

    pub use tokio_io::AsyncRead;
    pub use tokio_io::AsyncWrite;

    pub use tokio::runtime::Runtime;
    pub use tokio::spawn;

    pub use command_processor::CommandProcessor;
}

mod io {
    pub use tokio::io::AsyncRead;
    pub use tokio::io::AsyncWrite;
    pub use tokio::io::Error;

    pub use tokio_io::codec::Decoder;
    pub use tokio_io::codec::Encoder;

    pub mod net {
        pub use tokio::net::TcpListener;
        pub use tokio::net::TcpStream;
    }
}

use std::io::BufRead;
use std::net::SocketAddr;
use std::thread;

use async::AsyncRead;
use async::Future;
use async::Sink;
use async::Stream;
use io::net::TcpStream;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Logger, Root};

fn main() {
    init_log4rs();
    let address = "0.0.0.0:5500"
        .parse::<SocketAddr>()
        .expect("Failed to bind socket");

    info!(
        "Staring Server v{} on interface {}",
        env!("CARGO_PKG_VERSION"),
        address,
    );

    let server = server::Server::new(address).unwrap();
    let server = server.start().unwrap();

    info!("Server started successfully");

    let stdin = ::std::io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Err(_) => break,
            Ok(line) => match line.trim() {
                _ if line.is_empty() => {}
                "quit" | "q" | "exit" => break,
                _ => println!(":: Unknown command"),
            },
        }
    }
}

fn init_log4rs() {
    let stdout = ConsoleAppender::builder().build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("libmessages-sys", LevelFilter::Info))
        .logger(Logger::builder().build("libmessages", LevelFilter::Info))
        .logger(Logger::builder().build("mecview_server", LevelFilter::Trace))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .expect("Failed to create logger config");

    let _ = log4rs::init_config(config).expect("Failed to initialize logger");
}
