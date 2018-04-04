pub extern crate libmessages_sys as raw;

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
            asn_uper_decode(&mut raw::asn_DEF_ClientRegistration, buffer)?
        }))
    }

    pub fn encode(&self, target: &mut [u8]) -> Result<usize, ()> {
        match self {
            Message::Registration(ref registration) => unsafe {
                asn_uper_encode(&mut raw::asn_DEF_ClientRegistration, registration.as_ref(), target)
            }
        }
    }
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe {
            match self {
                Message::Registration(ref registration) => asn_free_content(&mut raw::asn_DEF_ClientRegistration, registration.as_ref()),
            };
        }
    }
}


unsafe fn asn_free_content<T>(asn_type: &mut raw::asn_TYPE_descriptor_t, value: &T) {
    asn_free(asn_type, value, true)
}

unsafe fn asn_free<T>(asn_type: &mut raw::asn_TYPE_descriptor_t, value: &T, only_content: bool) {
    trace!("asn_free type {:?}", asn_type);
    asn_type.free_struct.expect("free_struct is NULL, is the library not loaded?")(
        asn_type as *mut raw::asn_TYPE_descriptor_t,
        value as *const T as *mut ::std::os::raw::c_void,
        if only_content {1} else {0} as ::std::os::raw::c_int,
    );
    trace!("freed");
}

unsafe fn asn_uper_encode<T>(asn_type: &mut raw::asn_TYPE_descriptor_t, value: &T, buffer: &mut [u8]) -> Result<usize, ()> {
    let result = raw::uper_encode_to_buffer(
        asn_type as *mut raw::asn_TYPE_descriptor_t,
        value as *const T as *const ::std::os::raw::c_void as *mut ::std::os::raw::c_void,
        buffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
        buffer.len()
    );

    if result.encoded < 0 {
        trace!("{:?}", result);
        Err(())
    } else {
        trace!("fine");
        Ok(result.encoded as usize)
    }
}


unsafe fn asn_uper_decode<T>(asn_type: &mut raw::asn_TYPE_descriptor_t, buffer: &[u8]) -> Result<Box<T>, ()> {
    let mut pointer : *mut T = ::std::ptr::null_mut();
    let result = raw::uper_decode_complete(
        ::std::ptr::null_mut(),
        asn_type as *mut raw::asn_TYPE_descriptor_t,
        (&mut pointer as *mut *mut T) as *mut *mut ::std::os::raw::c_void,
        buffer.as_ptr() as *mut ::std::os::raw::c_void,
        buffer.len(),
    );

    if result.code != raw::asn_dec_rval_code_e::RC_OK {
        trace!("{:?}", result);
        Err(())
    } else {
        trace!("fine");
        Ok(Box::from_raw(pointer))
    }
}


#[cfg(test)]
mod tests {
    use std::ptr;
    use std::mem;
    use std::os::raw;
    use raw::*;
    use super::*;

    fn init_logger() {
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

    #[test]
    fn do_not_panic() {
        init_logger();
        unsafe {
            let mut buffer = [0u8; 32];
            let mut p = mem::zeroed::<PositionOffset>();
            p.position_north = 77 as raw::c_long;
            p.position_east = 77 as raw::c_long;
            p.std_dev_position_east = ptr::null_mut();
            p.std_dev_position_north = ptr::null_mut();

            trace!("{:?}", uper_encode_to_buffer(
                &mut asn_DEF_PositionOffset as *mut asn_TYPE_descriptor_s,
                &mut p as *mut _ as *mut raw::c_void,
                buffer.as_mut_ptr() as *mut raw::c_void,
                buffer.len() as usize
            ));
            let mut string = String::new();
            for byte in buffer.iter() {
                string.push_str(&format!("{:02x} ", byte));
            }
            trace!("{}", string)
        }
    }
}