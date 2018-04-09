
use std::io::Error;
use std::io::ErrorKind;
use std::net::SocketAddr;

use std::fmt::Debug;

use async::Sender;
use async::Sink;
use async::sink::Wait;
use async::CommandProcessor;


use adapter;
use adapter::Adapter;

pub struct Client<A: Debug+Send+Sized+'static, E: Debug+Sized+Send+'static, D: Adapter<E> + Send + 'static> {
    address: SocketAddr,
    adapter: D,
    algorithm: Sender<A>,
    variant: Variant,
    _e: ::std::marker::PhantomData<E>,
}

impl<A: Debug+Send+Sized+'static, E: Debug+Sized+Send+'static, D: Adapter<E> + Send + 'static> Client<A, E, D> {

    pub fn new(address: SocketAddr, adapter: D, algorithm: Sender<A>) -> Client<A, E, D> {
        Client {
            address,
            adapter,
            algorithm,
            variant: Variant::Unknown,
            _e: ::std::marker::PhantomData,
        }
    }

    fn init_as_sensor(&mut self) -> Result<(), Error> {
        trace!("Initializing as sensor");
        self.remote_unsubscribe()
    }

    fn init_as_vehicle(&mut self) -> Result<(), Error> {
        trace!("Initializing as vehicle");
        self.remote_init()
    }

    fn subscribe_to_environment_model(&mut self) -> Result<(), Error> {
        trace!("Subscribing to environment model");
        // TODO
        unimplemented!()
    }

    fn unsubscribe_from_environment_model(&mut self) -> Result<(), Error> {
        trace!("Unsubscribing from environment model");
        // TODO
        unimplemented!()
    }

    fn on_new_environment_model(&mut self, model: E) -> Result<(), Error> {
        trace!("New EnvironmentModel: {:?}", model);
        // self.remote.send(model) oder sowas
        // TODO
        unimplemented!()
    }

    fn remote_init(&mut self) -> Result<(), Error> {
        self.adapter.init_vehicle()
    }

    fn remote_unsubscribe(&mut self) -> Result<(), Error> {
        self.adapter.unsubscribe()
    }

    fn remote_subscribe(&mut self) -> Result<(), Error> {
        self.adapter.subscribe()
    }

    fn set_variant(&mut self, variant: Variant) {
        trace!("Client is now a {:?}", variant);
        self.variant = variant;
    }
}

impl<A: Debug+Send+Sized+'static, E: Debug+Sized+Send+'static, D: Adapter<E> + Send + 'static> Drop for Client<A, E, D> {
    fn drop(&mut self) {
        trace!("Dropping client with address {}", self.address);
    }
}

impl<A: Debug+Send+Sized+'static, E: Debug+Sized+Send+'static, D: Adapter<E> + Send + 'static> CommandProcessor<Command<A>> for Client<A, E, D> {
    fn process_command(&mut self, command: Command<A>) -> Result<(), Error> {
        match self.variant {
            Variant::Unknown => {
                if let Command::UpdateVariant(variant) = command {
                    self.set_variant(variant);
                    match variant {
                        Variant::Sensor  => self.init_as_sensor(),
                        Variant::Vehicle => self.init_as_vehicle(),
                        _ => Err(Error::from(ErrorKind::InvalidInput)),
                    }
                } else {
                    Err(Error::from(ErrorKind::InvalidInput))
                }
            },
            Variant::Sensor => {
                match command {
                    Command::SensorIsIdle => Ok(()), // great... I guess
                    _ => Err(Error::from(ErrorKind::InvalidInput)),
                }
            },
            Variant::Vehicle => match command {
                Command::Subscribe => self.subscribe_to_environment_model(),
                Command::Unsubscribe => self.unsubscribe_from_environment_model(),
                _ => Err(Error::from(ErrorKind::InvalidInput)),
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Variant {
    Unknown,
    Sensor,
    Vehicle,
}

#[derive(Debug, Clone)]
pub enum Command<A> {
    UpdateVariant(Variant),
    SensorIsIdle,
    Subscribe,
    Unsubscribe,
    UpdateAlgorithm(Box<A>),
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug)]
    struct M;

    #[test]
    fn test_default_variant_is_unknown() {
        let (sender, receiver) = ::async::channel(2);
        let mut client : Client<M> = Client::new(sender);
        assert_eq!(Variant::Unknown, client.variant);
    }

    #[test]
    fn test_update_variant_cannot_change_if_not_unknown() {
        let (sender, receiver) = ::async::channel(2);
        let mut client : Client<M> = Client::new(sender);
        client.variant = Variant::Vehicle;
        assert!(client.process_command(Command::UpdateVariant(Variant::Vehicle)).is_err());
        assert!(client.process_command(Command::UpdateVariant(Variant::Sensor)).is_err());
    }

    #[test]
    fn test_update_variant_for_client_type_sensor() {
        test_update_variant_for_client_type(Variant::Sensor);
    }

    #[test]
    fn test_update_variant_for_client_type_vehicle() {
        test_update_variant_for_client_type(Variant::Vehicle);
    }

    fn test_update_variant_for_client_type(variant: Variant) {
        let (sender, receiver) = ::async::channel(2);
        let mut client : Client<M> = Client::new(sender);
        assert_eq!((), client.process_command(Command::UpdateVariant(variant)).unwrap());
        assert_eq!(variant, client.variant);
    }
}