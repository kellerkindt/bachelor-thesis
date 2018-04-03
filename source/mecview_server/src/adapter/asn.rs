


use client;
use async::Sink;
use async::Future;
use async::Sender;
use async::CommandProcessor;

use libmessages::Message;
use libmessages::raw::ClientType;
use libmessages::raw::ClientType_t;

use adapter::Command;

const CLIENT_TYPE_SENSOR  : ClientType_t = ClientType::ClientType_sensor  as ClientType_t;
const CLIENT_TYPE_VEHICLE : ClientType_t = ClientType::ClientType_vehicle as ClientType_t;


pub struct AsnClientAdapter {
    client: Sender<client::Command>,
}

impl AsnClientAdapter {
    pub fn new(client: Sender<client::Command>) -> AsnClientAdapter {
        AsnClientAdapter {
            client,
        }
    }

    fn process_message(&mut self, message: Message) -> Result<(), ()> {
        match message {
            Message::Registration(registration) => {
                self.send_client(client::Command::UpdateVariant(
                    Self::variant_from_client_type_t(registration.type_)?
                ))
            }
        }
    }

    fn send_client(&mut self, command: client::Command) -> Result<(), ()> {
        // this blocks since its is important to know whether the client
        // is still alive and to have a bit back pressure on flooding requests
        match self.client.clone().send(command).wait() {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to send command: {:?}", e);
                Err(())
            }
        }
    }

    fn variant_from_client_type_t(t: ClientType_t) -> Result<client::Variant, ()> {
        match t {
            CLIENT_TYPE_SENSOR  => Ok(client::Variant::Sensor),
            CLIENT_TYPE_VEHICLE => Ok(client::Variant::Vehicle),
            _ => Err(()),
        }
    }
}

impl CommandProcessor<Command<Message>> for AsnClientAdapter {
    fn process_command(&mut self, command: Command<Message>) -> Result<(), ()> {
        match command {
            Command::ProcessMessage(message) => self.process_message(message),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn err_on_invalid_type_t_into_variant() {
        assert_eq!(Err(()), AsnClientAdapter::variant_from_client_type_t(0xFF as ClientType_t));
    }

    #[test]
    fn client_type_t_into_variant_sensor() {
        assert_eq!(Ok(client::Variant::Sensor), AsnClientAdapter::variant_from_client_type_t(CLIENT_TYPE_SENSOR));
    }

    #[test]
    fn client_type_t_into_variant_vehicle() {
        assert_eq!(Ok(client::Variant::Vehicle), AsnClientAdapter::variant_from_client_type_t(CLIENT_TYPE_VEHICLE));
    }
}