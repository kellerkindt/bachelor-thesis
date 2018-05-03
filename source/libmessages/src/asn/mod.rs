mod message;

pub extern crate libmessages_sys as raw;
pub use self::message::*;

pub mod uper;
pub mod xer;

#[cfg(test)]
mod tests {
    use super::super::log4rs;
    use super::super::RawMessage;
    use super::raw::*;
    use super::*;

    use std::mem;
    use std::os;
    use std::ptr;

    pub fn init_logger() {
        use log::LevelFilter;
        use log4rs::append::console::ConsoleAppender;
        use log4rs::config::Appender;
        use log4rs::config::Config;
        use log4rs::config::Root;
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

            trace!(
                "{:?}",
                uper_encode_to_buffer(
                    &mut asn_DEF_PositionOffset as *mut asn_TYPE_descriptor_s,
                    &mut p as *mut _ as *mut os::raw::c_void,
                    buffer.as_mut_ptr() as *mut os::raw::c_void,
                    buffer.len() as usize
                )
            );
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
        let raw = reg.try_encode_uper().unwrap();
        assert_eq!(1, raw.length());
        assert_eq!(raw::ClientRegistration::type_id(), raw.identifier());
        assert_eq!(&[0x20], raw.bytes());
    }

    #[test]
    fn basic_decode_client_registration() {
        let msg = RawMessage::new(raw::ClientRegistration::type_id(), vec![0x20]).unwrap();
        let reg = raw::ClientRegistration::decode(&msg).unwrap();
        assert_eq!(
            raw::ClientType_ClientType_vehicle as raw::ClientType_t,
            reg.type_
        );
    }

    #[test]
    fn decoding_of_invalid_data_fails() {
        let msg = RawMessage::new(raw::ClientRegistration::type_id(), vec![0xFF]).unwrap();
        let res = raw::ClientRegistration::decode(&msg);
        assert!(res.is_err());
    }

}
