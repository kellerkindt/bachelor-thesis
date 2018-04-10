
use std::io::Error;
use std::io::ErrorKind;
use std::net::SocketAddr;

use std::fmt::Debug;

use std::sync::Arc;

use async::Sender;
use async::CommandProcessor;

use adapter::Adapter;

use algorithm::Algorithm;

use messages::RawMessage;

pub struct Client<A: Debug+Send+Sized+'static, E: Debug+Sized+Sync+Send+'static, G: Algorithm<A, E, Identifier=SocketAddr>+Sized+'static, D: Adapter<E> + Send + 'static> {
    address: SocketAddr,
    myself: Sender<Command<A, E>>,
    adapter: D,
    algorithm: G,
    variant: Variant,
    _e: ::std::marker::PhantomData<E>,
}

impl<A: Debug+Send+Sized+'static, E: Debug+Sized+Send+Sync+'static, G: Algorithm<A, E, Identifier=SocketAddr>+Sized+'static, D: Adapter<E> + Send + 'static> Client<A, E, G, D> {

    pub fn new(myself: Sender<Command<A, E>>, address: SocketAddr, adapter: D, algorithm: G) -> Client<A, E, G, D> {
        Client {
            address,
            myself,
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

    fn subscribe_to_algorithm(&mut self) -> Result<(), Error> {
        trace!("Subscribing to algorithm");
        let mut sender = self.myself.clone();
        self.algorithm.subscribe_environment_model(self.address.clone(), Box::new(move |e| {
            match sender.try_send(Command::UpdateEnvironmentModel(e)) {
                Ok(_)  => Ok(()),
                Err(_) => Err(Error::from(ErrorKind::UnexpectedEof)),
            }
        }))
    }

    fn unsubscribe_from_algorithm(&mut self) -> Result<(), Error> {
        trace!("Unsubscribing from environment model");
        self.algorithm.unsubscribe_environment_model(self.address.clone())
    }

    fn update_environment_model(&mut self, model: Arc<RawMessage<E>>) -> Result<(), Error> {
        trace!("New EnvironmentModel: {:?}", model);
        self.adapter.update_environment_model(model)
    }

    fn update_algorithm(&mut self, update: Box<A>) -> Result<(), Error> {
        trace!("Updating algorithm: {:?}", update);
        self.algorithm.update(update)
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

impl<A: Debug+Send+Sized+'static, E: Debug+Sized+Send+Sync+'static, G: Algorithm<A, E, Identifier=SocketAddr>+Sized+'static, D: Adapter<E> + Send + 'static> Drop for Client<A, E, G, D> {
    fn drop(&mut self) {
        trace!("Dropping client with address {}", self.address);
    }
}

impl<A: Debug+Send+Sized+'static, E: Debug+Sized+Send+Sync+'static, G: Algorithm<A, E, Identifier=SocketAddr>+Sized+'static, D: Adapter<E> + Send + 'static> CommandProcessor<Command<A, E>> for Client<A, E, G, D> {
    fn process_command(&mut self, command: Command<A, E>) -> Result<(), Error> {
        trace!("Client/{}/{:?} is going to process command: {:?}", self.address, self.variant, command);
        let result = match self.variant {
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
                    Command::UpdateAlgorithm(update) => self.update_algorithm(update),
                    _ => Err(Error::from(ErrorKind::InvalidInput)),
                }
            },
            Variant::Vehicle => match command {
                Command::Subscribe => self.subscribe_to_algorithm(),
                Command::Unsubscribe => self.unsubscribe_from_algorithm(),
                Command::UpdateEnvironmentModel(model) => self.update_environment_model(model),
                _ => Err(Error::from(ErrorKind::InvalidInput)),
            },
        };
        trace!("Client/{} result: {:?}", self.address, result);
        result
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Variant {
    Unknown,
    Sensor,
    Vehicle,
}

#[derive(Debug, Clone)]
pub enum Command<A, E: Debug+Send+Sized+'static> {
    UpdateVariant(Variant),
    SensorIsIdle,
    Subscribe,
    Unsubscribe,
    UpdateAlgorithm(Box<A>),
    UpdateEnvironmentModel(Arc<RawMessage<E>>),
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug)]
    struct M;

    #[derive(Debug)]
    struct MockAdapter();
    impl<E: Debug> Adapter<E> for MockAdapter {
        fn init_vehicle(&mut self) -> Result<(), Error> {
            Ok(())
        }

        fn unsubscribe(&mut self) -> Result<(), Error> {
            Ok(())
        }

        fn subscribe(&mut self) -> Result<(), Error> {
            Ok(())
        }

        fn update_environment_model(&mut self, model: Arc<RawMessage<E>>) -> Result<(), Error> {
            Ok(())
        }
    }

    fn test_client() -> Client<M, M, impl Algorithm<M, M, Identifier=SocketAddr>, impl Adapter<M>> {
        let address = "0.0.0.0:2048".parse::<SocketAddr>().unwrap();
        let (sender, receiver) = ::async::channel(2);
        let (alg_tx, alg_rx) = ::async::channel(2);
        Client::new(sender, address, MockAdapter(), alg_tx)
    }

    #[test]
    fn test_default_variant_is_unknown() {
        assert_eq!(Variant::Unknown, test_client().variant);
    }

    #[test]
    fn test_update_variant_cannot_change_if_not_unknown() {
        let mut client = test_client();
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
        let mut client = test_client();
        assert_eq!((), client.process_command(Command::UpdateVariant(variant)).unwrap());
        assert_eq!(variant, client.variant);
    }
}