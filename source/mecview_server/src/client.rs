
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
    subscribed_model: bool,
    subscribed_count: bool,
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
            subscribed_model: false,
            subscribed_count: false,
            _e: ::std::marker::PhantomData,
        }
    }

    fn init_as_sensor(&mut self) -> Result<(), Error> {
        trace!("Client/{}/{:?} is initializing as sensor", self.address, self.variant);
        self.subscribe_to_algorithm_as_sensor()
    }

    fn init_as_vehicle(&mut self) -> Result<(), Error> {
        trace!("Client/{}/{:?} is initializing as vehicle", self.address, self.variant);
        self.remote_init()?;
        self.subscribe_to_algorithm_as_vehicle()
    }

    fn subscribe_to_algorithm_as_vehicle(&mut self) -> Result<(), Error> {
        trace!("Client/{}/{:?} is subscribing to algorithm", self.address, self.variant);
        let mut sender = self.myself.clone();
        self.algorithm.subscribe_environment_model(self.address.clone(), Box::new(move |e| {
            sender
                .try_send(Command::UpdateEnvironmentModel(e))
                .map_err(|_| Error::from(ErrorKind::UnexpectedEof))
        }))?;
        self.subscribed_model = true;
        Ok(())
    }

    fn unsubscribe_from_algorithm_as_vehicle(&mut self) -> Result<(), Error> {
        trace!("Client/{}/{:?} is unsubscribing from algorithm", self.address, self.variant);
        self.algorithm.unsubscribe_environment_model(self.address.clone())?;
        self.subscribed_model = false;
        Ok(())
    }

    fn activate_algorithm_model_subscription(&mut self) -> Result<(), Error> {
        trace!("Client/{}/{:?} is activating model subscription", self.address, self.variant);
        self.algorithm.activate_environment_model_subscription(self.address.clone())
    }


    fn deactivate_algorithm_model_subscription(&mut self) -> Result<(), Error> {
        trace!("Client/{}/{:?} is deactivating model subscription", self.address, self.variant);
        self.algorithm.deactivate_environment_model_subscription(self.address.clone())
    }


    fn subscribe_to_algorithm_as_sensor(&mut self) -> Result<(), Error> {
        trace!("Client/{}/{:?} is subscribing to algorithm", self.address, self.variant);
        let mut sender = self.myself.clone();
        self.algorithm.subscribe_listener_count(self.address.clone(), Box::new(move |count| {
            let command = if count > 0 {
                Command::RemoteSubscribe
            } else {
                Command::RemoteUnsubscribe
            };
            sender
                .try_send(command)
                .map_err(|_| Error::from(ErrorKind::UnexpectedEof))
        }))?;
        self.subscribed_count = true;
        Ok(())
    }

    fn unsubscribe_from_algorithm_as_sensor(&mut self) -> Result<(), Error> {
        trace!("Client/{}/{:?} is unsubscribing from algorithm", self.address, self.variant);
        self.algorithm.unsubscribe_listener_count(self.address.clone())?;
        self.subscribed_count = false;
        Ok(())
    }

    fn update_environment_model(&mut self, model: Arc<RawMessage<E>>) -> Result<(), Error> {
        trace!("Client/{}/{:?} received environment model: {:?}", self.address, self.variant, model);
        self.adapter.update_environment_model(model)
    }

    fn update_algorithm(&mut self, update: Box<A>) -> Result<(), Error> {
        trace!("Client/{}/{:?} received algorithm update: {:?}", self.address, self.variant, update);
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
        trace!("Client/{}/{:?} is going to be a {:?}", self.address, self.variant, variant);
        self.variant = variant;
    }
}

impl<A: Debug+Send+Sized+'static, E: Debug+Sized+Send+Sync+'static, G: Algorithm<A, E, Identifier=SocketAddr>+Sized+'static, D: Adapter<E> + Send + 'static> Drop for Client<A, E, G, D> {
    fn drop(&mut self) {
        info!("Client/{}/{:?} is going to be dropped", self.address, self.variant);
        if self.subscribed_model {
            trace!("Client/{}/{:?}: Remote did not unsubscribe from algorithm.model", self.address, self.variant);
            let _ = self.unsubscribe_from_algorithm_as_vehicle();
        }
        if self.subscribed_count {
            trace!("Client/{}/{:?}: Remote did not unsubscribe from algorithm.count", self.address, self.variant);
            let _ = self.unsubscribe_from_algorithm_as_sensor();
        }
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
                    Command::RemoteSubscribe => self.remote_subscribe(),
                    Command::RemoteUnsubscribe => self.remote_unsubscribe(),
                    _ => Err(Error::from(ErrorKind::InvalidInput)),
                }
            },
            Variant::Vehicle => match command {
                Command::Subscribe => self.activate_algorithm_model_subscription(),
                Command::Unsubscribe => self.deactivate_algorithm_model_subscription(),
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
    RemoteSubscribe,
    RemoteUnsubscribe,
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