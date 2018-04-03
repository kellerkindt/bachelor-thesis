
use std::sync::Arc;

use libmessages::Message;
use libmessages::raw::ClientType;
use libmessages::raw::ClientType_t;
use libmessages::raw::ClientRegistration;

use futures::Stream;
use futures::future::Future;
use futures::sync::mpsc::SendError;
use futures::sync::mpsc::Receiver;

pub struct Client {
    variant: Variant,
}

impl Client {
    pub fn process_all(mut self, mut receiver: Receiver<Arc<Message>>) -> impl Future<Item=(),Error=()>  {
        receiver.for_each(move |message| {
            self.process(message)
        })
    }

    pub fn process(&mut self, message: Arc<Message>) -> Result<(), ()> {
        match self.variant {
            Variant::Unknown => self.message_on_variant_unknown(message),
            Variant::Sensor  => Err(()),
            Variant::Vehicle => Err(()),
        }
    }

    fn message_on_variant_unknown(&mut self, message: Arc<Message>) -> Result<(), ()> {
        match *message {
            Message::Registration(registration) => {
                self.update_variant_for_client_type(registration.type_)
            },
            _ => {
                error!("Invalid message: {:?}", message);
                Err(())
            }
        }
    }

    fn update_variant_for_client_type(&mut self, client_type: ClientType_t) -> Result<(), ()> {
        match ClientType::from(client_type) {
            Some(ClientType::ClientType_sensor) => {
                self.set_variant(Variant::Sensor);
                Ok(())
            },
            Some(ClientType::ClientType_vehicle) => {
                self.set_variant(Variant::Vehicle);
                Ok(())
            },
            _ => {
                error!("Invalid ClientType_t: {:?}", client_type);
                Err(())
            }
        }
    }

    fn set_variant(&mut self, variant: Variant) {
        trace!("Client is now a {:?}", variant);
        self.variant = variant;
    }
}

impl Default for Client {
    fn default() -> Self {
        Client {
            variant: Variant::Unknown,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Variant {
    Unknown,
    Sensor,
    Vehicle,
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_variant_is_unknown() {
        assert_eq!(Variant::Unknown, Client::default().variant);
    }

    #[test]
    fn test_update_for_client_type_sensor() {
        test_update_for_client_type(Variant::Sensor, ClientType::ClientType_sensor);
    }

    #[test]
    fn test_update_for_client_type_vehicle() {
        test_update_for_client_type(Variant::Vehicle, ClientType::ClientType_vehicle);
    }

    fn test_update_for_client_type(variant: Variant, client_type: ClientType) {
        let mut client = Client::default();
        assert_eq!(Ok(()), client.update_variant_for_client_type(client_type as ClientType_t));
        assert_eq!(variant, client.variant);
    }
}