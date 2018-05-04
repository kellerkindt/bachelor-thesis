#[macro_use]
extern crate log;
extern crate clap;
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
use std::net::IpAddr;
use std::net::SocketAddr;

use log::LevelFilter;
use log::SetLoggerError;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::Handle;

use clap::App;
use clap::Arg;

struct ServerConfig {
    ip: IpAddr,
    port: u16,
    log: Option<LevelFilter>,
    init_message: Option<String>,
}

fn main() {
    let config = parse_config();
    let _ = init_log4rs(config.log).expect("Failed to initialize logger");
    let address = SocketAddr::new(config.ip, config.port);

    info!(
        "Staring Server v{} on interface {}",
        env!("CARGO_PKG_VERSION"),
        address,
    );

    let mut server = server::Server::new(address).unwrap();

    if let Some(path) = config.init_message {
        info!("Loading InitMessage from {}", path);
        let _ = server.load_init_message(path).expect("Loading failed");
    }

    let _server = server.start().unwrap();
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

fn parse_config() -> ServerConfig {
    let parser = create_argument_parser();
    let matches = parser.get_matches();
    ServerConfig {
        ip: matches
            .value_of("interface")
            .unwrap()
            .parse::<IpAddr>()
            .unwrap(),
        port: matches.value_of("port").unwrap().parse::<u16>().unwrap(),
        log: matches
            .value_of("log")
            .and_then(|v| match v {
                "trace" => Some(LevelFilter::Trace),
                "debug" => Some(LevelFilter::Debug),
                "info" => Some(LevelFilter::Info),
                "warn" => Some(LevelFilter::Warn),
                "err" => Some(LevelFilter::Error),
                _ => panic!("Invalid log level"),
            })
            .or(None),
        init_message: matches.value_of("init_message").map(|s| String::from(s)),
    }
}

fn create_argument_parser<'a, 'b>() -> App<'a, 'b> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("NUMBER")
                .help("Sets the TCP listen port")
                .takes_value(true)
                .default_value("2000"),
        )
        .arg(
            Arg::with_name("interface")
                .short("i")
                .long("interface")
                .value_name("INTERFACE")
                .help("Sets the interface to listen on")
                .takes_value(true)
                .default_value("0.0.0.0"),
        )
        .arg(
            Arg::with_name("log")
                .short("l")
                .long("log")
                .value_name("LEVEL")
                .help("Sets the log level of the server")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("init_message")
                .short("v")
                .long("init-message")
                .value_name("PATH")
                .help("The path to the InitMessage to send to a Vehicle")
                .takes_value(true),
        )
}

pub(crate) fn init_log4rs(level: Option<LevelFilter>) -> Result<Handle, SetLoggerError> {
    let stdout = ConsoleAppender::builder().build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("libmessages-sys", level.unwrap_or(LevelFilter::Info)))
        .logger(Logger::builder().build("libmessages", level.unwrap_or(LevelFilter::Info)))
        .logger(Logger::builder().build("mecview_server", level.unwrap_or(LevelFilter::Trace)))
        .build(
            Root::builder()
                .appender("stdout")
                .build(level.unwrap_or(LevelFilter::Info)),
        )
        .expect("Failed to create logger config");

    log4rs::init_config(config)
}
