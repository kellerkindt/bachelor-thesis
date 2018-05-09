#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use client;

use std::io::Error;
use std::io::ErrorKind;

use std::sync::Arc;

use async::sink::Wait;
use async::Sink;

use messages::asn::raw::*;
use messages::asn::AsnMessage;
use messages::asn::Generalize;
use messages::asn::Message;
use messages::RawMessage;

use adapter::Adapter;

const CLIENT_TYPE_SENSOR: ClientType_t = ClientType_ClientType_sensor as ClientType_t;
const CLIENT_TYPE_VEHICLE: ClientType_t = ClientType_ClientType_vehicle as ClientType_t;

pub struct AsnAdapter<
    E: Sink<SinkItem = Arc<RawMessage<Message>>, SinkError = Error> + Send + 'static,
> {
    encoder: Wait<E>,
    raw_init_message: Option<Arc<RawMessage<Message>>>,
}

impl<E: Sink<SinkItem = Arc<RawMessage<Message>>, SinkError = Error> + Send + 'static>
    AsnAdapter<E>
{
    pub fn new(encoder: E, init_message: Option<Arc<RawMessage<Message>>>) -> AsnAdapter<E> {
        AsnAdapter {
            encoder: encoder.wait(),
            raw_init_message: init_message,
        }
    }

    fn new_subscribe_message(status: SubscriptionStatus_t) -> UpdateSubscription {
        let mut update = UpdateSubscription::default();
        update.subscription_status = status;
        update
    }

    #[allow(unknown_lints)]
    #[allow(needless_pass_by_value)]
    fn remote_send<M: AsnMessage>(&mut self, message: M) -> Result<(), Error> {
        match message.try_encode_uper() {
            Err(_) => Err(Error::from(ErrorKind::InvalidData)),
            Ok(raw) => {
                self.encoder.send(Arc::new(raw.generalize()))?;
                self.encoder.flush()
            }
        }
    }

    fn remote_send_raw(&mut self, message: Arc<RawMessage<Message>>) -> Result<(), Error> {
        self.encoder.send(message)?;
        self.encoder.flush()
    }
}

pub fn map_message(
    client_id: usize,
    message: Message,
) -> Result<client::Command<SensorFrame, EnvironmentFrame>, Error> {
    match message {
        Message::ClientRegistration(ref reg) => Ok(client::Command::UpdateVariant(
            variant_from_client_registration(reg)?,
        )),
        Message::UpdateSubscription(ref update) => match update.subscription_status
            as SubscriptionStatus
        {
            SubscriptionStatus_SubscriptionStatus_subscribed => Ok(client::Command::Subscribe),
            SubscriptionStatus_SubscriptionStatus_unsubscribed => Ok(client::Command::Unsubscribe),
            _ => Err(Error::from(ErrorKind::NotFound)),
        },
        Message::SensorFrame(mut frame) => {
            frame.envelope.sender_id = client_id as ::std::os::raw::c_long;
            Ok(client::Command::UpdateAlgorithm(frame))
        },
        Message::EnvironmentFrame(_) => Err(Error::from(ErrorKind::InvalidInput)),
        Message::RoadClearanceFrame(_) => Err(Error::from(ErrorKind::InvalidInput)),
        Message::SensorIdleFrame(_) => Ok(client::Command::SensorIsIdle),
        Message::UpdateStatus(_) => Err(Error::from(ErrorKind::InvalidInput)),
        Message::InitMessage(_) => Err(Error::from(ErrorKind::InvalidInput)),
    }
}

impl<E: Sink<SinkItem = Arc<RawMessage<Message>>, SinkError = Error> + Send + 'static>
    Adapter<EnvironmentFrame> for AsnAdapter<E>
{
    fn init_vehicle(&mut self) -> Result<(), Error> {
        if let Some(init) = self.raw_init_message.clone() {
            self.remote_send_raw(init)
        } else {
            self.remote_send(InitMessage::default())
        }
    }

    #[allow(unknown_lints)]
    #[allow(cast_lossless)]
    fn unsubscribe(&mut self) -> Result<(), Error> {
        self.remote_send(Self::new_subscribe_message(
            SubscriptionStatus_SubscriptionStatus_unsubscribed as SubscriptionStatus_t,
        ))
    }

    #[allow(unknown_lints)]
    #[allow(cast_lossless)]
    fn subscribe(&mut self) -> Result<(), Error> {
        self.remote_send(Self::new_subscribe_message(
            SubscriptionStatus_SubscriptionStatus_subscribed as SubscriptionStatus_t,
        ))
    }

    fn update_environment_model(
        &mut self,
        model: Arc<RawMessage<EnvironmentFrame>>,
    ) -> Result<(), Error> {
        self.remote_send_raw(model.generalize())
    }
}

fn variant_from_client_registration(r: &ClientRegistration) -> Result<client::Variant, Error> {
    variant_from_client_type_t(r.type_)
}

fn variant_from_client_type_t(t: ClientType_t) -> Result<client::Variant, Error> {
    match t {
        CLIENT_TYPE_SENSOR => Ok(client::Variant::Sensor),
        CLIENT_TYPE_VEHICLE => Ok(client::Variant::Vehicle),
        _ => Err(Error::from(ErrorKind::NotFound)),
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn err_on_invalid_type_t_into_variant() {
        assert!(variant_from_client_type_t(0xFF as ClientType_t).is_err());
    }

    #[test]
    fn client_type_t_into_variant_sensor() {
        assert_eq!(
            client::Variant::Sensor,
            variant_from_client_type_t(CLIENT_TYPE_SENSOR).unwrap()
        );
    }

    #[test]
    fn client_type_t_into_variant_vehicle() {
        assert_eq!(
            client::Variant::Vehicle,
            variant_from_client_type_t(CLIENT_TYPE_VEHICLE).unwrap()
        );
    }
}
