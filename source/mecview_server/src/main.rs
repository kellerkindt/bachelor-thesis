#![feature(impl_trait)]

#[macro_use]
extern crate log;
extern crate log4rs;

extern crate tokio;
extern crate futures;

extern crate libmessages;

mod client;
mod adapter;
mod command_processor;

mod async {
    pub use futures::Future;
    pub use futures::Stream;
    pub use futures::Sink;
    pub use futures::sync::mpsc::Sender;
    pub use futures::sync::mpsc::SendError;
    pub use futures::sync::mpsc::Receiver;
    pub use futures::sync::mpsc::channel;

    pub use command_processor::CommandProcessor;
}