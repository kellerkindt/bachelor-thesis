mod manager;
mod vehicle;

pub use self::vehicle::*;
pub use self::manager::*;

use std::fmt::Debug;
use std::io::Error;
use std::io::ErrorKind;

use std::sync::Arc;

use async::CommandProcessor;
use async::Sender;

use messages::RawMessage;

pub type CountListener = Box<FnMut(usize, usize) -> Result<(), Error> + Send + 'static>;
pub type EnvironmentListener<E> =
    Box<FnMut(Arc<RawMessage<E>>) -> Result<(), Error> + Send + 'static>;

pub enum Command<A: Send, E: Send, I: Debug + Send + Sized + 'static> {
    Update(Box<A>),
    Publish(RawMessage<E>),
    SubscribeEnvironmentModel(I, EnvironmentListener<E>),
    UnsubscribeEnvironmentModel(I),
    SubscribeListenerCount(I, CountListener),
    UnsubscribeListenerCount(I),
    ActivateEnvironmentModelSubscription(I),
    DeactivateEnvironmentModelSubscription(I),
}

pub trait Algorithm<A: Send + Debug, E: Send + Debug> {
    type Identifier: Send + PartialEq + 'static;

    fn update(&mut self, update: Box<A>) -> Result<(), Error>;

    fn publish(&mut self, model: RawMessage<E>) -> Result<(), Error>;

    fn subscribe_environment_model(
        &mut self,
        identifier: Self::Identifier,
        listener: EnvironmentListener<E>,
    ) -> Result<(), Error>;

    fn unsubscribe_environment_model(&mut self, identifier: Self::Identifier) -> Result<(), Error>;

    fn activate_environment_model_subscription(
        &mut self,
        identifier: Self::Identifier,
    ) -> Result<(), Error>;

    fn deactivate_environment_model_subscription(
        &mut self,
        identifier: Self::Identifier,
    ) -> Result<(), Error>;

    fn subscribe_listener_count(
        &mut self,
        identifier: Self::Identifier,
        listener: CountListener,
    ) -> Result<(), Error>;

    fn unsubscribe_listener_count(&mut self, identifier: Self::Identifier) -> Result<(), Error>;
}

impl<A: Send + Debug, E: Send + Debug, I: PartialEq + Debug + Send + Sized + 'static>
    Algorithm<A, E> for Sender<Command<A, E, I>>
{
    type Identifier = I;

    fn update(&mut self, update: Box<A>) -> Result<(), Error> {
        self.try_send(Command::Update(update))
            .map_err(|_| Error::from(ErrorKind::UnexpectedEof))
    }

    fn publish(&mut self, model: RawMessage<E>) -> Result<(), Error> {
        self.try_send(Command::Publish(model))
            .map_err(|_| Error::from(ErrorKind::UnexpectedEof))    }


    fn subscribe_environment_model(
        &mut self,
        identifier: I,
        listener: EnvironmentListener<E>,
    ) -> Result<(), Error> {
        self.try_send(Command::SubscribeEnvironmentModel(identifier, listener))
            .map_err(|_| Error::from(ErrorKind::UnexpectedEof))
    }

    fn unsubscribe_environment_model(&mut self, identifier: I) -> Result<(), Error> {
        self.try_send(Command::UnsubscribeEnvironmentModel(identifier))
            .map_err(|_| Error::from(ErrorKind::UnexpectedEof))
    }

    fn activate_environment_model_subscription(
        &mut self,
        identifier: <Self as Algorithm<A, E>>::Identifier,
    ) -> Result<(), Error> {
        self.try_send(Command::ActivateEnvironmentModelSubscription(identifier))
            .map_err(|_| Error::from(ErrorKind::UnexpectedEof))
    }

    fn deactivate_environment_model_subscription(
        &mut self,
        identifier: <Self as Algorithm<A, E>>::Identifier,
    ) -> Result<(), Error> {
        self.try_send(Command::DeactivateEnvironmentModelSubscription(identifier))
            .map_err(|_| Error::from(ErrorKind::UnexpectedEof))
    }

    fn subscribe_listener_count(
        &mut self,
        identifier: <Self as Algorithm<A, E>>::Identifier,
        listener: CountListener,
    ) -> Result<(), Error> {
        self.try_send(Command::SubscribeListenerCount(identifier, listener))
            .map_err(|_| Error::from(ErrorKind::UnexpectedEof))
    }

    fn unsubscribe_listener_count(
        &mut self,
        identifier: <Self as Algorithm<A, E>>::Identifier,
    ) -> Result<(), Error> {
        self.try_send(Command::UnsubscribeListenerCount(identifier))
            .map_err(|_| Error::from(ErrorKind::UnexpectedEof))
    }
}

impl<
        A: Send + Debug,
        E: Send + Debug,
        I: PartialEq + Debug + Send + Sized + 'static,
        G: Algorithm<A, E, Identifier = I>,
    > CommandProcessor<Command<A, E, I>> for G
{
    fn process_command(&mut self, command: Command<A, E, I>) -> Result<(), Error> {
        trace!("Received command");
        let result = match command {
            Command::Update(model) => self.update(model),
            Command::Publish(model) => self.publish(model),
            Command::SubscribeEnvironmentModel(id, listener) => {
                self.subscribe_environment_model(id, listener)
            }
            Command::UnsubscribeEnvironmentModel(id) => self.unsubscribe_environment_model(id),
            Command::SubscribeListenerCount(id, listener) => {
                self.subscribe_listener_count(id, listener)
            }
            Command::UnsubscribeListenerCount(id) => self.unsubscribe_listener_count(id),
            Command::ActivateEnvironmentModelSubscription(id) => {
                self.activate_environment_model_subscription(id)
            }
            Command::DeactivateEnvironmentModelSubscription(id) => {
                self.deactivate_environment_model_subscription(id)
            }
        };
        trace!("  :: result: {:?}", result);
        result
    }
}
