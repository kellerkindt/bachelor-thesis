mod message;

pub extern crate libmessages_sys as raw;
pub use self::message::*;




pub fn uper_encode_to_new_buffer<T>(asn_type: &mut raw::asn_TYPE_descriptor_t, value: &T) -> Result<Vec<u8>, ()> {
    let mut pointer : *mut u8 = ::std::ptr::null_mut();
    let result = unsafe {
        raw::uper_encode_to_new_buffer(
            asn_type as *mut raw::asn_TYPE_descriptor_t,
            0 as *mut raw::asn_per_constraints_s,
            value as *const T as *mut T as *mut ::std::os::raw::c_void,
            (&mut pointer as *mut *mut u8) as *mut *mut ::std::os::raw::c_void,
        )
    };
    if result < 0 {
        Err(())
    } else {
        Ok(unsafe {
            Vec::from_raw_parts(pointer, result as usize, result as usize)
        })
    }
}

pub unsafe fn uper_encode<T>(asn_type: &mut raw::asn_TYPE_descriptor_t, value: &T, buffer: &mut [u8]) -> Result<usize, ()> {
    let result = raw::uper_encode_to_buffer(
        asn_type as *mut raw::asn_TYPE_descriptor_t,
        value as *const T as *const ::std::os::raw::c_void as *mut ::std::os::raw::c_void,
        buffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
        buffer.len()
    );

    trace!("result: {:?} for type: {:?}", result, asn_type);
    if result.encoded < 0 {
        warn!("Encoding failed: {:?}", result);
        Err(())
    } else {
        trace!("fine");
        // encoded is the length in BITS not bytes...
        Ok((result.encoded as usize + 7) / 8)
    }
}

pub unsafe fn uper_decode<T>(asn_type: &mut raw::asn_TYPE_descriptor_t, buffer: &[u8]) -> Result<Box<T>, ()> {
    let mut pointer : *mut T = ::std::ptr::null_mut();
    let result = raw::uper_decode_complete(
        ::std::ptr::null_mut(),
        asn_type as *mut raw::asn_TYPE_descriptor_t,
        (&mut pointer as *mut *mut T) as *mut *mut ::std::os::raw::c_void,
        buffer.as_ptr() as *mut ::std::os::raw::c_void,
        buffer.len(),
    );

    trace!("result: {:?} for type: {:?}", result, asn_type);
    if result.code != raw::asn_dec_rval_code_e_RC_OK {
        warn!("Decoding failed: {:?}", result);
        Err(())
    } else {
        trace!("fine");
        Ok(Box::from_raw(pointer))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::raw::*;
    use super::super::RawMessage;
    use super::super::log4rs;

    use std::ptr;
    use std::mem;
    use std::os;

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
            .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
            .unwrap();

        let _ = log4rs::init_config(config);
    }


    #[test]
    fn library_link_valid_do_not_panic() {
        init_logger();
        unsafe {
            let mut buffer = [0u8; 32];
            let mut p = mem::zeroed::<PositionOffset>();
            p.position_north = 77 as os::raw::c_long;
            p.position_east = 77 as os::raw::c_long;
            p.std_dev_position_east = ptr::null_mut();
            p.std_dev_position_north = ptr::null_mut();

            trace!("{:?}", uper_encode_to_buffer(
                &mut asn_DEF_PositionOffset as *mut asn_TYPE_descriptor_s,
                &mut p as *mut _ as *mut os::raw::c_void,
                buffer.as_mut_ptr() as *mut os::raw::c_void,
                buffer.len() as usize
            ));
            let mut string = String::new();
            for byte in buffer.iter() {
                string.push_str(&format!("{:02x} ", byte));
            }
            trace!("{}", string)
        }
    }

    #[test]
    fn basic_encode_client_registration() {
        let mut reg = raw::ClientRegistration::default();
        reg.type_ = raw::ClientType_ClientType_vehicle as raw::ClientType_t;
        let raw = reg.encode().unwrap();
        assert_eq!(1, raw.length());
        assert_eq!(raw::ClientRegistration::type_id(), raw.identifier());
        assert_eq!(&[0x20], raw.bytes());
    }

    #[test]
    fn basic_decode_client_registration() {
        let msg = RawMessage::new(raw::ClientRegistration::type_id(), vec![0x20]).unwrap();
        let reg = raw::ClientRegistration::decode(&msg).unwrap();
        assert_eq!(raw::ClientType_ClientType_vehicle as raw::ClientType_t, reg.type_);
    }

}