
use std::io::Error;
use std::io::ErrorKind;
use std::fmt::Debug;

use std::sync::Arc;

use async::Sender;

use messages::RawMessage;

pub enum Command<A, E, I: Debug+Send+Sized+'static> {
    Update(Box<A>),
    Subscribe(I, Box<FnMut(Arc<RawMessage<E>>) -> Result<(), Error>+Send+'static>),
    Unsubscribe(I)
}

pub trait Algorithm<A: Debug, E: Debug> {
    type Identifier;

    fn update(&mut self, update: Box<A>) -> Result<(), Error>;

    fn subscribe(&mut self, identifier: Self::Identifier, sink: Box<FnMut(Arc<RawMessage<E>>) -> Result<(), Error>+Send+'static>) -> Result<(), Error>;

    fn unsubscribe(&mut self, identifier: Self::Identifier) -> Result<(), Error>;
}

impl<A: Debug, E: Debug+Send, I: Debug+Send+Sized+'static> Algorithm<A, E> for Sender<Command<A, E, I>> {
    type Identifier = I;

    fn update(&mut self, update: Box<A>) -> Result<(), Error> {
        match self.try_send(Command::Update(update)) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::from(ErrorKind::UnexpectedEof)),
        }
    }

    fn subscribe(&mut self, identifier: I, sink: Box<FnMut(Arc<RawMessage<E>>) -> Result<(), Error>+Send+'static>) -> Result<(), Error> {
        match self.try_send(Command::Subscribe(identifier, sink)) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::from(ErrorKind::UnexpectedEof)),
        }
    }

    fn unsubscribe(&mut self, identifier: I) -> Result<(), Error> {
        match self.try_send(Command::Unsubscribe(identifier)) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::from(ErrorKind::UnexpectedEof)),
        }
    }
}