

use io;
use client;


use async::Sink;
use async::Async;
use async::Future;
use async::Sender;
use async::Stream;
use async::CommandProcessor;

use io::AsyncRead;
use io::net::TcpStream;

use libmessages::Message;
use libmessages::raw::ClientType;
use libmessages::raw::ClientType_t;

use bytes::BytesMut;
use byteorder::ByteOrder;
use byteorder::NetworkEndian;

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
            Message::Registration(ref registration) => {
                self.send_client(client::Command::UpdateVariant(
                    Self::variant_from_client_type_t(unsafe {(*registration).type_})?
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

const ASN_HEADER_SIZE : usize = 8;

struct AsnDecoder {
    buffer: BytesMut,
    socket: TcpStream,
}

impl AsnDecoder {
    fn fill_buffer(&mut self) -> Result<Async<()>, io::Error> {
        loop {
            self.buffer.reserve(ASN_HEADER_SIZE);

            let _ = try_ready!(self.socket.read_buf(&mut self.buffer));

            if self.buffer.len() >= ASN_HEADER_SIZE {
                break;
            }
        }

        let message_size = NetworkEndian::read_u32(&self.buffer[..4]);
        let message_size_with_header = message_size as usize + ASN_HEADER_SIZE;

        if message_size_with_header < self.buffer.len() {
            if self.buffer.capacity() < message_size_with_header {
                let new_to_reserve = message_size_with_header - self.buffer.capacity();
                self.buffer.reserve(new_to_reserve);
            }
            loop {
                if 0 == try_ready!(self.socket.read_buf(&mut self.buffer)) {
                    break;
                }
            }
        }
        return Ok(Async::Ready(()));
    }
}

impl Stream for AsnDecoder {
    type Item = Message;
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        let is_ready = self.fill_buffer()?.is_ready();
        if is_ready {
            let message_size = NetworkEndian::read_u32(&self.buffer[0..4]);
            let message_type = NetworkEndian::read_u32(&self.buffer[4..8]);


            unsafe {
                let mut reg : *mut ::libmessages::raw::ClientRegistration = 0 as *mut ::libmessages::raw::ClientRegistration;
                let result = ::libmessages::raw::uper_decode(
                    0 as *mut ::libmessages::raw::asn_codec_ctx_s,
                    &mut ::libmessages::raw::asn_DEF_ClientRegistration as *mut ::libmessages::raw::asn_TYPE_descriptor_s,
                    &mut (reg as *mut ::std::os::raw::c_void),
                    self.buffer[8..].as_ptr() as *mut ::std::os::raw::c_void,
                    message_size as usize,
                    0,
                    0
                );
            }

            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
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