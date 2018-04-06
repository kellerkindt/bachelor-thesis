
use client;

use std::io::Error;
use std::io::ErrorKind;

use async::Sink;
use async::Future;
use async::Sender;
use async::sink::Wait;
use async::CommandProcessor;

use io::Encoder;
use io::Decoder;

use libmessages::Message;
use libmessages::raw::*;

use bytes::BufMut;
use bytes::BytesMut;
use byteorder::ByteOrder;
use byteorder::NetworkEndian;

use adapter::Command;

const CLIENT_TYPE_SENSOR  : ClientType_t = ClientType_ClientType_sensor  as ClientType_t;
const CLIENT_TYPE_VEHICLE : ClientType_t = ClientType_ClientType_vehicle as ClientType_t;


pub struct AsnClientAdapter<E: Sink<SinkItem=Message,SinkError=Error> + Send + 'static> {
    encoder: Wait<E>,
    client:  Sender<client::Command>,
}

impl<E: Sink<SinkItem=Message,SinkError=Error> + Send + 'static> AsnClientAdapter<E> {
    pub fn new(encoder: E, client: Sender<client::Command>) -> AsnClientAdapter<E> {
        AsnClientAdapter {
            encoder: encoder.wait(),
            client,
        }
    }

    fn process_message(&mut self, message: Message) -> Result<(), Error> {
        match message {
            Message::Registration(ref registration) => {
                self.send_client(client::Command::UpdateVariant(
                    Self::variant_from_client_registration(registration)?
                ))
            },
            _ => {
                error!("Not implemented: {:?}", message);
                Err(Error::from(ErrorKind::InvalidInput))
            }
        }
    }

    fn send_client(&mut self, command: client::Command) -> Result<(), Error> {
        // this blocks since its is important to know whether the client
        // is still alive and to have a bit back pressure on flooding requests
        match self.client.clone().send(command).wait() {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::from(ErrorKind::UnexpectedEof))
        }
    }

    fn variant_from_client_registration(r: &ClientRegistration) -> Result<client::Variant, Error> {
        Self::variant_from_client_type_t(r.type_)
    }

    fn variant_from_client_type_t(t: ClientType_t) -> Result<client::Variant, Error> {
        match t {
            CLIENT_TYPE_SENSOR  => Ok(client::Variant::Sensor),
            CLIENT_TYPE_VEHICLE => Ok(client::Variant::Vehicle),
            _ => Err(Error::from(ErrorKind::InvalidInput)),
        }
    }
}

impl<E: Sink<SinkItem=Message,SinkError=Error> + Send + 'static> CommandProcessor<Command<Message>> for AsnClientAdapter<E> {
    fn process_command(&mut self, command: Command<Message>) -> Result<(), Error> {
        match command {
            Command::ProcessMessage(message) => self.process_message(message),
            Command::SendMessage(message) => match self.encoder.send(message) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::from(ErrorKind::UnexpectedEof)),
            }
        }
    }
}

const ASN_HEADER_SIZE : usize = 8;

pub struct AsnCodec();


impl Decoder for AsnCodec {
    type Item = Message;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<<Self as Decoder>::Item>, <Self as Decoder>::Error> {
        trace!("len: {}", src.len());
        if src.len() > ASN_HEADER_SIZE {
            let message_size = NetworkEndian::read_u32(&src[0..4]) as usize;
            let message_type = NetworkEndian::read_u32(&src[4..8]);
            let total_size = ASN_HEADER_SIZE + message_size;

            if src.len() >= total_size {
                debug!("Trying to decode, size={}, type={}", message_type, message_size);
                let buffer = src.split_to(total_size);
                match Message::decode(message_type, &buffer[ASN_HEADER_SIZE..]) {
                    Err(_) => {
                        debug!("Failed to decode");
                        Err(Error::from(ErrorKind::InvalidData))
                    },
                    Ok(message) => {
                        debug!("Successfully decoded: {:?}", message);
                        Ok(Some(message))
                    },
                }
            } else {
                let src_capacity = src.capacity();
                src.reserve((message_size + ASN_HEADER_SIZE) - src_capacity);
                Ok(None)
            }
        } else {
            src.reserve(ASN_HEADER_SIZE);
            Ok(None)
        }
    }
}

impl Encoder for AsnCodec {
    type Item = Message;
    type Error = Error;

    fn encode(&mut self, item: <Self as Encoder>::Item, dst: &mut BytesMut) -> Result<(), <Self as Encoder>::Error> {
        trace!("Trying to encode: {:?}", item);
        let mut buffer = [0u8; 1024*1024]; // TODO
        let message_size = match item.encode(&mut buffer[..]) {
            Err(_) => {
                trace!("Failed to encode: {:?}", item);
                return Err(Error::from(ErrorKind::Other))
            },
            Ok(size) => size as u32
        };

        let total_size = ASN_HEADER_SIZE + message_size as usize;
        dst.reserve(total_size);
        dst.put_u32::<NetworkEndian>(message_size);
        dst.put_u32::<NetworkEndian>(item.type_id());
        let slice = &buffer[..message_size as usize];
        dst.put_slice(slice);
        trace!("Successfully Encoded: {:?}", item);
        Ok(())
    }
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode_simple() {
        use ::bytes::BytesMut;
        use ::std::io::Cursor;

        let mut buf = BytesMut::with_capacity(1024);
        AsnCodec().encode(Message::decode_client_registration(&[0x20, 0x00, 0x00]).unwrap(), &mut buf).unwrap();
        assert_eq!(&[
            0x00, 0x00, 0x00, 0x03, // length
            0x00, 0x00, 0x00, 0x01, // type
            0x20                    // message
        ], &buf[..9]);
    }

    #[test]
    fn test_decode_underflow() {
        let mut buffer = BytesMut::with_capacity(ASN_HEADER_SIZE-1);
        assert!(AsnCodec().decode(&mut buffer).unwrap().is_none());
    }

    #[test]
    fn test_decode_simple() {
        use ::bytes::BytesMut;
        use ::std::io::Cursor;

        let mut buf = BytesMut::with_capacity(1024);
        buf.put_slice(&[
            0x00, 0x00, 0x00, 0x03, // length
            0x00, 0x00, 0x00, 0x01, // type
            0x20,                   // message
        ]);
        let message = AsnCodec().decode(&mut buf).unwrap().unwrap();
        let mut buffer = [0u8; 10];
        assert_eq!(Ok(1), message.encode(&mut buffer[..]));
        assert_eq!(&[0x20], &buffer[..1])
    }

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