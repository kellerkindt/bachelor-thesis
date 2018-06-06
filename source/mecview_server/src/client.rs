use std::io::Error;
use std::io::ErrorKind;
use std::net::SocketAddr;

use std::fmt::Debug;

use std::sync::Arc;

use async::Sender;

use adapter::Adapter;

use libalgorithm::Algorithm;
use libmessages::RawMessage;

pub struct Client<
    S: Debug + Send + Sized + 'static,
    A: Algorithm<S, SocketAddr> + Sized + 'static,
    D: Adapter + Send + 'static,
> {
    address: SocketAddr,
    myself: Sender<Command<S>>,
    adapter: D,
    algorithm: A,
    variant: Variant,
    subscribed_model: bool,
    subscribed_count: bool,
}

impl<
        S: Debug + Send + Sized + 'static,
        G: Algorithm<S, SocketAddr> + Sized + 'static,
        D: Adapter + Send + 'static,
    > Client<S, G, D>
{
    pub fn new(
        myself: Sender<Command<S>>,
        address: SocketAddr,
        adapter: D,
        algorithm: G,
    ) -> Client<S, G, D> {
        Client {
            address,
            myself,
            adapter,
            algorithm,
            variant: Variant::Unknown,
            subscribed_model: false,
            subscribed_count: false,
        }
    }

    fn init_as_sensor(&mut self) -> Result<(), Error> {
        trace!(
            "Client/{}/{:?} is initializing as sensor",
            self.address,
            self.variant
        );
        self.subscribe_to_algorithm_as_sensor();
        Ok(())
    }

    fn init_as_vehicle(&mut self) -> Result<(), Error> {
        trace!(
            "Client/{}/{:?} is initializing as vehicle",
            self.address,
            self.variant
        );
        self.remote_init()?;
        self.subscribe_to_algorithm_as_vehicle();
        Ok(())
    }

    fn subscribe_to_algorithm_as_vehicle(&mut self) {
        trace!(
            "Client/{}/{:?} is subscribing to algorithm",
            self.address,
            self.variant
        );
        let mut sender = self.myself.clone();
        self.algorithm.subscribe_environment_model(
            self.address,
            Box::new(move |e| {
                trace!("Trying to send UpdateEnvironmentModel");
                sender
                    .try_send(Command::UpdateEnvironmentModel(e))
                    .map_err(|_| Error::from(ErrorKind::UnexpectedEof))
            }),
        );
        self.subscribed_model = true;
    }

    fn unsubscribe_from_algorithm_as_vehicle(&mut self) {
        trace!(
            "Client/{}/{:?} is unsubscribing from algorithm",
            self.address,
            self.variant
        );
        self.algorithm.unsubscribe_environment_model(self.address);
        self.subscribed_model = false;
    }

    fn activate_algorithm_model_subscription(&mut self) {
        trace!(
            "Client/{}/{:?} is activating model subscription",
            self.address,
            self.variant
        );
        self.algorithm
            .activate_environment_model_subscription(self.address);
    }

    fn deactivate_algorithm_model_subscription(&mut self) {
        trace!(
            "Client/{}/{:?} is deactivating model subscription",
            self.address,
            self.variant
        );
        self.algorithm
            .deactivate_environment_model_subscription(self.address);
    }

    fn subscribe_to_algorithm_as_sensor(&mut self) {
        trace!(
            "Client/{}/{:?} is subscribing to algorithm",
            self.address,
            self.variant
        );
        let mut sender = self.myself.clone();
        self.algorithm.subscribe_listener_count(
            self.address,
            Box::new(move |before, after| {
                let command = if before == 0 && after > 0 {
                    Command::RemoteSubscribe
                } else if before > 0 && after == 0 {
                    Command::RemoteUnsubscribe
                } else {
                    return Ok(());
                };
                sender
                    .try_send(command)
                    .map_err(|_| Error::from(ErrorKind::UnexpectedEof))
            }),
        );
        self.subscribed_count = true;
    }

    fn unsubscribe_from_algorithm_as_sensor(&mut self) {
        trace!(
            "Client/{}/{:?} is unsubscribing from algorithm",
            self.address,
            self.variant
        );
        self.algorithm.unsubscribe_listener_count(self.address);
        self.subscribed_count = false;
    }

    fn update_environment_model(&mut self, model: Arc<RawMessage>) -> Result<(), Error> {
        trace!(
            "Client/{}/{:?} received environment model: {:?}",
            self.address,
            self.variant,
            model
        );
        self.adapter.update_environment_model(model)
    }

    fn update_algorithm(&mut self, update: Box<S>) {
        trace!(
            "Client/{}/{:?} received algorithm update: {:?}",
            self.address,
            self.variant,
            update
        );
        self.algorithm.update(update);
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
        trace!(
            "Client/{}/{:?} is going to be a {:?}",
            self.address,
            self.variant,
            variant
        );
        self.variant = variant;
    }
}

impl<
        A: Debug + Send + Sized + 'static,
        G: Algorithm<A, SocketAddr> + Sized + 'static,
        D: Adapter + Send + 'static,
    > Drop for Client<A, G, D>
{
    fn drop(&mut self) {
        info!(
            "Client/{}/{:?} is going to be dropped",
            self.address, self.variant
        );
        if self.subscribed_model {
            trace!(
                "Client/{}/{:?}: Remote did not unsubscribe from algorithm.model",
                self.address,
                self.variant
            );
            let _ = self.unsubscribe_from_algorithm_as_vehicle();
        }
        if self.subscribed_count {
            trace!(
                "Client/{}/{:?}: Remote did not unsubscribe from algorithm.count",
                self.address,
                self.variant
            );
            let _ = self.unsubscribe_from_algorithm_as_sensor();
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
pub enum Command<S> {
    UpdateVariant(Variant),
    SensorIsIdle,
    Subscribe,
    Unsubscribe,
    UpdateAlgorithm(Box<S>),
    UpdateEnvironmentModel(Arc<RawMessage>),
    RemoteSubscribe,
    RemoteUnsubscribe,
}

impl<S: Debug + Send + Sized + 'static> Command<S> {
    pub fn apply<A: Algorithm<S, SocketAddr> + Sized + 'static, D: Adapter + Send + 'static>(
        self,
        client: &mut Client<S, A, D>,
    ) -> Result<(), Error> {
        trace!(
            "Client/{}/{:?} is going to process command: {:?}",
            client.address,
            client.variant,
            self
        );
        let result = match client.variant {
            Variant::Unknown => {
                if let Command::UpdateVariant(variant) = self {
                    client.set_variant(variant);
                    match variant {
                        Variant::Sensor => client.init_as_sensor(),
                        Variant::Vehicle => client.init_as_vehicle(),
                        _ => Err(Error::from(ErrorKind::InvalidInput)),
                    }
                } else {
                    Err(Error::from(ErrorKind::InvalidInput))
                }
            }
            Variant::Sensor => {
                match self {
                    Command::SensorIsIdle => Ok(()), // great... I guess
                    Command::UpdateAlgorithm(update) => {
                        client.update_algorithm(update);
                        Ok(())
                    }
                    Command::RemoteSubscribe => client.remote_subscribe(),
                    Command::RemoteUnsubscribe => client.remote_unsubscribe(),
                    _ => Err(Error::from(ErrorKind::InvalidInput)),
                }
            }
            Variant::Vehicle => match self {
                Command::Subscribe => {
                    client.activate_algorithm_model_subscription();
                    Ok(())
                }
                Command::Unsubscribe => {
                    client.deactivate_algorithm_model_subscription();
                    Ok(())
                }
                Command::UpdateEnvironmentModel(model) => {
                    client.update_environment_model(model);
                    Ok(())
                }
                _ => Err(Error::from(ErrorKind::InvalidInput)),
            },
        };
        trace!("Client/{} result: {:?}", client.address, result);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use async::Receiver;
    use async::Stream;
    use libalgorithm::CountListener;
    use libalgorithm::EnvironmentListener;
    use std::ops::IndexMut;

    #[derive(Default, Debug)]
    struct M();

    #[derive(Default, Debug)]
    struct MockAdapter {
        init_vehicle: usize,
        unsubscribe: usize,
        subscribe: usize,
        update_environment_model: Vec<Arc<RawMessage>>,
    }

    impl MockAdapter {
        fn assert(
            &self,
            init_vehicle: usize,
            unsubscribe: usize,
            subscribe: usize,
            update_environment_model: usize,
        ) {
            assert_eq!(init_vehicle, self.init_vehicle, "init_vehicle");
            assert_eq!(unsubscribe, self.unsubscribe, "unsubscribe");
            assert_eq!(subscribe, self.subscribe, "subscribe");
            assert_eq!(
                update_environment_model,
                self.update_environment_model.len(),
                "update_environment_model",
            );
        }

        fn clear(&mut self) {
            self.init_vehicle = 0;
            self.unsubscribe = 0;
            self.subscribe = 0;
            self.update_environment_model.clear();
        }
    }
    impl Adapter for MockAdapter {
        fn init_vehicle(&mut self) -> Result<(), Error> {
            self.init_vehicle += 1;
            Ok(())
        }

        fn unsubscribe(&mut self) -> Result<(), Error> {
            self.unsubscribe += 1;
            Ok(())
        }

        fn subscribe(&mut self) -> Result<(), Error> {
            self.subscribe += 1;
            Ok(())
        }

        fn update_environment_model(&mut self, model: Arc<RawMessage>) -> Result<(), Error> {
            self.update_environment_model.push(model);
            Ok(())
        }
    }

    #[derive(Default)]
    struct MockAlgorithm<A: Send + Debug> {
        update: Vec<Box<A>>,
        publish: Vec<RawMessage>,
        subscribe_environment_model: Vec<(SocketAddr, EnvironmentListener)>,
        unsubscribe_environment_model: Vec<(SocketAddr)>,
        activate_environment_model_subscription: Vec<SocketAddr>,
        deactivate_environment_model_subscription: Vec<SocketAddr>,
        subscribe_listener_count: Vec<(SocketAddr, CountListener)>,
        unsubscribe_listener_count: Vec<SocketAddr>,
    }
    impl<A: Send + Debug> MockAlgorithm<A> {
        fn assert(
            &self,
            update: usize,
            publish: usize,
            subscribe_environment_model: usize,
            unsubscribe_environment_model: usize,
            activate_environment_model_subscription: usize,
            deactivate_environment_model_subscription: usize,
            subscribe_listener_count: usize,
            unsubscribe_listener_count: usize,
        ) {
            assert_eq!(update, self.update.len(), "update");
            assert_eq!(publish, self.publish.len(), "publish");
            assert_eq!(
                subscribe_environment_model,
                self.subscribe_environment_model.len(),
                "subscribe_environment_model"
            );
            assert_eq!(
                unsubscribe_environment_model,
                self.unsubscribe_environment_model.len(),
                "unsubscribe_environment_model"
            );
            assert_eq!(
                activate_environment_model_subscription,
                self.activate_environment_model_subscription.len(),
                "activate_environment_model_subscription"
            );
            assert_eq!(
                deactivate_environment_model_subscription,
                self.deactivate_environment_model_subscription.len(),
                "deactivate_environment_model_subscription"
            );
            assert_eq!(
                subscribe_listener_count,
                self.subscribe_listener_count.len(),
                "subscribe_listener_count"
            );
            assert_eq!(
                unsubscribe_listener_count,
                self.unsubscribe_listener_count.len(),
                "unsubscribe_listener_count"
            );
        }

        fn clear(&mut self) {
            self.update.clear();
            self.publish.clear();
            self.subscribe_environment_model.clear();
            self.unsubscribe_environment_model.clear();
            self.activate_environment_model_subscription.clear();
            self.deactivate_environment_model_subscription.clear();
            self.subscribe_listener_count.clear();
            self.unsubscribe_listener_count.clear();
        }
    }
    impl<A: Send + Debug> Algorithm<A, SocketAddr> for MockAlgorithm<A> {
        fn update(&mut self, update: Box<A>) {
            self.update.push(update);
        }

        fn publish(&mut self, model: RawMessage) {
            self.publish.push(model);
        }

        fn subscribe_environment_model(
            &mut self,
            identifier: SocketAddr,
            listener: EnvironmentListener,
        ) {
            self.subscribe_environment_model
                .push((identifier, listener));
        }

        fn unsubscribe_environment_model(&mut self, identifier: SocketAddr) {
            self.unsubscribe_environment_model.push(identifier);
        }

        fn activate_environment_model_subscription(&mut self, identifier: SocketAddr) {
            self.activate_environment_model_subscription
                .push(identifier);
        }

        fn deactivate_environment_model_subscription(&mut self, identifier: SocketAddr) {
            self.deactivate_environment_model_subscription
                .push(identifier);
        }

        fn subscribe_listener_count(&mut self, identifier: SocketAddr, listener: CountListener) {
            self.subscribe_listener_count.push((identifier, listener));
        }

        fn unsubscribe_listener_count(&mut self, identifier: SocketAddr) {
            self.unsubscribe_listener_count.push(identifier);
        }
    }

    fn test_client() -> (
        Receiver<Command<M>>,
        Client<M, MockAlgorithm<M>, MockAdapter>,
    ) {
        let _ = ::init_log4rs(Some(::log::LevelFilter::Trace));
        let address = "0.0.0.0:2048".parse::<SocketAddr>().unwrap();
        let (sender, receiver) = ::async::channel(128);
        (
            receiver,
            Client::new(
                sender,
                address,
                MockAdapter::default(),
                MockAlgorithm::default(),
            ),
        )
    }

    fn variant_client(
        variant: Variant,
        cleared: bool,
    ) -> (
        ::std::sync::mpsc::Receiver<Command<M>>,
        Client<M, MockAlgorithm<M>, MockAdapter>,
    ) {
        let (receiver, mut client) = test_client();
        assert!(
            client
                .process_command(Command::UpdateVariant(variant))
                .is_ok()
        );
        if cleared {
            client.algorithm.clear();
            client.adapter.clear();
        }
        let (tx, rx) = ::std::sync::mpsc::channel();
        ::std::thread::spawn(move || {
            ::tokio::run(receiver.for_each(move |c| {
                tx.send(c).unwrap();
                Ok(())
            }));
        });
        (rx, client)
    }

    fn apply_all_commands<A: Sync + Send + Debug + 'static>(
        receiver: &mut ::std::sync::mpsc::Receiver<Command<A>>,
        client: &mut Client<A, MockAlgorithm<A>, MockAdapter>,
    ) {
        // TODO bad!
        ::std::thread::sleep(::std::time::Duration::from_millis(1_000));
        for _ in 0..5 {
            while let Ok(command) = receiver.try_recv() {
                client.process_command(command).unwrap();
            }
        }
    }

    #[test]
    fn test_default_variant_is_unknown() {
        assert_eq!(Variant::Unknown, test_client().1.variant);
    }

    #[test]
    fn test_update_variant_cannot_change_if_not_unknown() {
        let (_, mut client) = test_client();
        client.variant = Variant::Vehicle;
        assert!(
            client
                .process_command(Command::UpdateVariant(Variant::Vehicle))
                .is_err()
        );
        assert!(
            client
                .process_command(Command::UpdateVariant(Variant::Sensor))
                .is_err()
        );
    }

    #[test]
    fn test_sensor_being_registered_at_algorithm() {
        let (_, mut client) = test_client();
        assert!(
            client
                .process_command(Command::UpdateVariant(Variant::Sensor))
                .is_ok()
        );
        client.algorithm.assert(0, 0, 0, 0, 0, 0, 1, 0);
        assert_eq!(
            client.address,
            client.algorithm.subscribe_listener_count[0].0,
        );
    }

    #[test]
    fn test_sensor_remote_subscribing_unsubscribe() {
        let (_, mut client) = variant_client(Variant::Sensor, true);
        assert!(client.process_command(Command::RemoteSubscribe).is_ok());
        client.algorithm.assert(0, 0, 0, 0, 0, 0, 0, 0);
        client.adapter.assert(0, 0, 1, 0);
        client.adapter.clear();
        assert!(client.process_command(Command::RemoteUnsubscribe).is_ok());
        client.adapter.assert(0, 1, 0, 0);
    }

    #[test]
    fn test_sensor_subscribe_unsubscribe_forbidden() {
        let (_, mut client) = variant_client(Variant::Sensor, true);
        assert!(client.process_command(Command::Subscribe).is_err());
        assert!(client.process_command(Command::Unsubscribe).is_err());
        client.algorithm.assert(0, 0, 0, 0, 0, 0, 0, 0);
        client.adapter.assert(0, 0, 0, 0);
    }

    #[test]
    fn test_sensor_remote_subscribe_unsubscribe_on_count_change() {
        let (mut receiver, mut client) = variant_client(Variant::Sensor, false);
        client.adapter.assert(0, 0, 0, 0);
        client.adapter.clear();
        client.algorithm.assert(0, 0, 0, 0, 0, 0, 1, 0);

        assert!(client.algorithm.subscribe_listener_count.index_mut(0).1(0, 1).is_ok());
        apply_all_commands(&mut receiver, &mut client);
        client.adapter.assert(0, 0, 1, 0);

        assert!(client.algorithm.subscribe_listener_count.index_mut(0).1(1, 0).is_ok());
        apply_all_commands(&mut receiver, &mut client);
        client.adapter.assert(0, 1, 1, 0);
    }

    #[test]
    fn test_sensor_update_algorithm() {
        let (_, mut client) = variant_client(Variant::Sensor, true);
        assert!(
            client
                .process_command(Command::UpdateAlgorithm(Box::new(M())))
                .is_ok()
        );
        client.adapter.assert(0, 0, 0, 0);
        client.algorithm.assert(1, 0, 0, 0, 0, 0, 0, 0);
    }

    #[test]
    fn test_vehicle_update_environment_model() {
        let (_, mut client) = variant_client(Variant::Vehicle, true);
        assert!(
            client
                .process_command(Command::UpdateEnvironmentModel(Arc::new(
                    RawMessage::new(0, Vec::new()).unwrap()
                )))
                .is_ok()
        );
        client.adapter.assert(0, 0, 0, 1);
        client.algorithm.assert(0, 0, 0, 0, 0, 0, 0, 0);
    }

    #[test]
    fn test_vehicle_activate_deactivate_algorithm_model_subscription() {
        let (_, mut client) = variant_client(Variant::Vehicle, true);
        assert!(client.process_command(Command::Subscribe).is_ok());
        client.adapter.assert(0, 0, 0, 0);
        client.algorithm.assert(0, 0, 0, 0, 1, 0, 0, 0);
        assert!(client.process_command(Command::Unsubscribe).is_ok());
        client.adapter.assert(0, 0, 0, 0);
        client.algorithm.assert(0, 0, 0, 0, 1, 1, 0, 0);
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
        let (_, mut client) = test_client();
        assert!(
            client
                .process_command(Command::UpdateVariant(variant))
                .is_ok()
        );
        assert_eq!(variant, client.variant);
    }
}
