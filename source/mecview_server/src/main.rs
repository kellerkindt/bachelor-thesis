
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