
use std::io::Error;
use std::io::ErrorKind;

use async::Sender;
use async::Sink;
use async::sink::Wait;
use async::CommandProcessor;

use adapter;

pub struct Client<M: ::std::fmt::Debug> {
    remote:  Wait<Sender<adapter::Command<M>>>,
    variant: Variant,
}

impl<M: ::std::fmt::Debug> Client<M> {

    pub fn new(remote: Sender<adapter::Command<M>>) -> Client<M> {
        Client {
            remote: remote.wait(),
            variant: Variant::Unknown,
        }
    }

    fn init_as_sensor(&mut self) -> Result<(), Error> {
        trace!("Initializing as sensor");
        // TODO
        Ok(())
    }

    fn init_as_vehicle(&mut self) -> Result<(), Error> {
        trace!("Initializing as vehicle");
        // TODO
        Ok(())
    }

    fn subscribe_to_environment_model(&mut self) -> Result<(), Error> {
        trace!("Subscribing to environment model");
        // TODO
        Ok(())
    }

    fn unsubscribe_from_environment_model(&mut self) -> Result<(), Error> {
        trace!("Unsubscribing from environment model");
        // TODO
        Ok(())
    }

    fn on_new_environment_model(&mut self, model: M) -> Result<(), Error> {
        trace!("New EnvironmentModel: {:?}", model);
        // self.remote.send(model) oder sowas
        // TODO
        Ok(())
    }

    fn set_variant(&mut self, variant: Variant) {
        trace!("Client is now a {:?}", variant);
        self.variant = variant;
    }
}

impl<M: ::std::fmt::Debug> CommandProcessor<Command> for Client<M> {
    fn process_command(&mut self, command: Command) -> Result<(), Error> {
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
pub enum Command {
    UpdateVariant(Variant),
    SensorIsIdle,
    Subscribe,
    Unsubscribe,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_variant_is_unknown() {
        assert_eq!(Variant::Unknown, Client::default().variant);
    }

    #[test]
    fn test_update_variant_cannot_change_if_not_unknown() {
        let mut client = Client::default();
        client.variant = Variant::Vehicle;
        assert_eq!(Err(()), client.process_command(Command::UpdateVariant(Variant::Vehicle)));
        assert_eq!(Err(()), client.process_command(Command::UpdateVariant(Variant::Sensor)));
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
        let mut client = Client::default();
        assert_eq!(Ok(()), client.process_command(Command::UpdateVariant(variant)));
        assert_eq!(variant, client.variant);
    }
}