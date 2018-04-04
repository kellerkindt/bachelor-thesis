pub extern crate libmessages_sys as raw;

mod asn;

#[macro_use]
extern crate log;

#[cfg(test)]
extern crate log4rs;

#[derive(Debug)]
pub enum Message {
    Registration(Box<raw::ClientRegistration>),
}

impl Message {
    pub fn decode_client_registration(buffer: &[u8]) -> Result<Message, ()> {
        Ok(Message::Registration(unsafe {
            asn::uper_decode(&mut raw::asn_DEF_ClientRegistration, buffer)?
        }))
    }

    pub fn encode(&self, target: &mut [u8]) -> Result<usize, ()> {
        match self {
            Message::Registration(ref registration) => unsafe {
                asn::uper_encode(&mut raw::asn_DEF_ClientRegistration, registration.as_ref(), target)
            }
        }
    }
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe {
            match self {
                Message::Registration(ref registration) => asn::free_content(&mut raw::asn_DEF_ClientRegistration, registration.as_ref()),
            };
        }
    }
}




#[cfg(test)]
mod tests {
    use std::ptr;
    use std::mem;
    use std::os::raw;
    use raw::*;
    use super::*;

    pub fn init_logger() {
        use log::LevelFilter;
        use log4rs::config::Config;
        use log4rs::config::Root;
        use log4rs::config::Appender;
        use log4rs::append::console::ConsoleAppender;
        use log4rs::encode::pattern::PatternEncoder;

        let encoder = PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {T} {M}:{L} {l} - {m}{n}");

        let appender = ConsoleAppender::builder()
            .encoder(Box::new(encoder))
            .build();

        let config = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(appender)))
            .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
            .unwrap();

        let _ = log4rs::init_config(config);
    }

    #[test]
    fn encode_client_registration() {
        init_logger();
        let registration = Message::Registration(unsafe {
            let mut registration = Box::new(mem::zeroed::<ClientRegistration>());
            registration.type_ = ClientType::ClientType_vehicle as ClientType_t;
            registration.covered_area           = ptr::null_mut();
            registration.minimum_message_period = ptr::null_mut();
            registration
        });
        let mut buffer = [0u8; 16];
        assert_eq!(Ok(3), registration.encode(&mut buffer[..]));
        assert_eq!(&[0x20_u8, 0x00, 0x00], &buffer[..3]);
        let mut string = String::new();
        for byte in buffer[..3].iter() {
            string.push_str(&format!("{:02x} ", byte));
        }
        trace!("{}", string);
    }

    #[test]
    fn decode_client_registration() {
        init_logger();
        let message = Message::decode_client_registration(&[0x20, 0x00, 0x00]);
        trace!("message: {:?}", message);
        assert!(message.is_ok());
        match message.expect("Message decoding failed") {
            Message::Registration(ref reg) => assert_eq!(ClientType::ClientType_vehicle as ClientType_t, reg.type_),
        }
        /*
        if let Message::Registration(ref reg) = message {
            assert_eq!(ClientType::ClientType_vehicle as ClientType_t, reg.type_);
        }*/
    }
}