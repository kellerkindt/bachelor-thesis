
mod sample;
pub use self::sample::*;

use std::io::Error;
use std::io::ErrorKind;
use std::fmt::Debug;

use std::sync::Arc;

use async::Sender;
use async::CommandProcessor;

use messages::RawMessage;

pub enum Command<A: Send, E: Send, I: Debug+Send+Sized+'static> {
    Update(Box<A>),
    SubscribeEnvironmentModel(I, Box<FnMut(Arc<RawMessage<E>>) -> Result<(), Error>+Send+'static>),
    UnsubscribeEnvironmentModel(I),
    SubscribeListenerCount(I, Box<FnMut(usize) -> Result<(), Error>+Send+'static>),
    UnsubscribeListenerCount(I),
}

pub trait Algorithm<A: Send+Debug, E: Send+Debug> {
    type Identifier : Send+PartialEq+'static;

    fn update(&mut self, update: Box<A>) -> Result<(), Error>;

    fn subscribe_environment_model(&mut self, identifier: Self::Identifier, sink: Box<FnMut(Arc<RawMessage<E>>) -> Result<(), Error>+Send+'static>) -> Result<(), Error>;

    fn unsubscribe_environment_model(&mut self, identifier: Self::Identifier) -> Result<(), Error>;

    fn subscribe_listener_count(&mut self, identifier: Self::Identifier, sink: Box<FnMut(usize) -> Result<(), Error>+Send+'static>) -> Result<(), Error>;

    fn unsubscribe_listener_count(&mut self, identifier: Self::Identifier) -> Result<(), Error>;
}

impl<A: Send+Debug, E: Send+Debug, I: PartialEq+Debug+Send+Sized+'static> Algorithm<A, E> for Sender<Command<A, E, I>> {
    type Identifier = I;

    fn update(&mut self, update: Box<A>) -> Result<(), Error> {
        match self.try_send(Command::Update(update)) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::from(ErrorKind::UnexpectedEof)),
        }
    }

    fn subscribe_environment_model(&mut self, identifier: I, sink: Box<FnMut(Arc<RawMessage<E>>) -> Result<(), Error>+Send+'static>) -> Result<(), Error> {
        match self.try_send(Command::SubscribeEnvironmentModel(identifier, sink)) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::from(ErrorKind::UnexpectedEof)),
        }
    }

    fn unsubscribe_environment_model(&mut self, identifier: I) -> Result<(), Error> {
        match self.try_send(Command::UnsubscribeEnvironmentModel(identifier)) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::from(ErrorKind::UnexpectedEof)),
        }
    }

    fn subscribe_listener_count(&mut self, identifier: <Self as Algorithm<A, E>>::Identifier, sink: Box<FnMut(usize) -> Result<(), Error>+Send+'static>) -> Result<(), Error> {
        match self.try_send(Command::SubscribeListenerCount(identifier, sink)) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::from(ErrorKind::UnexpectedEof)),
        }
    }

    fn unsubscribe_listener_count(&mut self, identifier: <Self as Algorithm<A, E>>::Identifier) -> Result<(), Error> {
        match self.try_send(Command::UnsubscribeListenerCount(identifier)) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::from(ErrorKind::UnexpectedEof)),
        }
    }
}

impl<A: Send+Debug, E: Send+Debug, I: PartialEq+Debug+Send+Sized+'static, G: Algorithm<A, E, Identifier=I>> CommandProcessor<Command<A, E, I>> for G {
    fn process_command(&mut self, command: Command<A, E, I>) -> Result<(), Error> {
        trace!("Received command");
        let result = match command {
            Command::Update(model) => self.update(model),
            Command::SubscribeEnvironmentModel(id, sink) => self.subscribe_environment_model(id, sink),
            Command::UnsubscribeEnvironmentModel(id) => self.unsubscribe_environment_model(id),
            Command::SubscribeListenerCount(id, sink) => self.subscribe_listener_count(id, sink),
            Command::UnsubscribeListenerCount(id) => self.unsubscribe_listener_count(id),
        };
        trace!("  :: result: {:?}", result);
        result
    }
}