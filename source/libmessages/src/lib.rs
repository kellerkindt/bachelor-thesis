pub extern crate libmessages_sys as raw;

mod asn;

#[macro_use]
extern crate log;

#[cfg(test)]
extern crate log4rs;

#[derive(Debug)]
pub enum Message {
    Registration(Box<raw::ClientRegistration>),
    UpdateSubscription(Box<raw::UpdateSubscription>),
    SensorFrame(Box<raw::SensorFrame>),
}


impl Message {
    pub fn decode_client_registration(buffer: &[u8]) -> Result<Message, ()> {
        Ok(Message::Registration(unsafe {
            asn::uper_decode(&mut raw::asn_DEF_ClientRegistration, buffer)?
        }))
    }

    pub fn decode_update_subscription(buffer: &[u8]) -> Result<Message, ()> {
        Ok(Message::UpdateSubscription(unsafe {
            asn::uper_decode(&mut raw::asn_DEF_UpdateSubscription, buffer)?
        }))
    }

    pub fn decode_sensor_frame(buffer: &[u8]) -> Result<Message, ()> {
        Ok(Message::SensorFrame(unsafe {
            asn::uper_decode(&mut raw::asn_DEF_SensorFrame, buffer)?
        }))
    }

    pub fn encode(&self, target: &mut [u8]) -> Result<usize, ()> {
        match self {
            Message::Registration(ref registration) => unsafe {
                asn::uper_encode(&mut raw::asn_DEF_ClientRegistration, registration.as_ref(), target)
            },
            Message::UpdateSubscription(ref subscription) => unsafe {
                asn::uper_encode(&mut raw::asn_DEF_UpdateSubscription, subscription.as_ref(), target)
            },
            Message::SensorFrame(ref frame) => unsafe {
                asn::uper_encode(&mut raw::asn_DEF_SensorFrame, frame.as_ref(), target)
            }
        }
    }
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe {
            match self {
                Message::Registration(ref value) => asn::free_content(&mut raw::asn_DEF_ClientRegistration, value.as_ref()),
                Message::UpdateSubscription(ref value) => asn::free_content(&mut raw::asn_DEF_UpdateSubscription, value.as_ref()),
                Message::SensorFrame(ref value) => asn::free_content(&mut raw::asn_DEF_SensorFrame, value.as_ref()),
            };
        }
    }
}




#[cfg(test)]
mod tests {
    use std::ptr;
    use std::mem;
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

    fn test_encode(message: Message, should_be: &[u8]) {
        let mut buffer = vec![0u8; should_be.len()];
        let result = message.encode(&mut buffer[..]);
        trace!("result {:?}", result);
        if let Ok(count) = result {
            let mut string = String::new();
            for byte in buffer[..count].iter() {
                string.push_str(&format!("{:02x} ", byte));
            }
            trace!("{}", string);
        }
        assert_eq!(Ok(should_be.len()), result);
        assert_eq!(should_be, &buffer[..]);
    }

    #[test]
    fn encode_sensor_frame() {
        init_logger();
        let frame = Message::SensorFrame(unsafe {
            let mut frame = Box::new(mem::zeroed::<SensorFrame>());
            raw::asn_set_empty(&mut frame.object_detections as *mut _ as *mut ::std::os::raw::c_void);
            frame.header.timestamp = 100;
            frame.envelope.version = 1;
            frame.envelope.sensor_type = raw::SensorType_SensorType_lidar as SensorType_t;
            frame.envelope.sender_id = 0;
            frame.envelope.pole_id = 2;
            frame.envelope.reference_point.latitude = 484010822;
            frame.envelope.reference_point.longitude = 99876076;
            frame.envelope.reference_point.altitude = 256000;
            frame
        });
        test_encode(frame, &[
            0x00, 0x00, 0x00, 0xc8, 0x02, 0x00, 0x15, 0x27, 0xe5, 0x44, 0x67, 0x13, 0xdc, 0xee, 0xc4,
            0x97, 0xc8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ]);
    }

    #[test]
    fn decode_sensor_frame() {
        init_logger();
        let message = Message::decode_sensor_frame(&[
            0x00, 0x00, 0x00, 0xc8, 0x02, 0x00, 0x15, 0x27, 0xe5, 0x44, 0x67, 0x13, 0xdc, 0xee, 0xc4,
            0x97, 0xc8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ]);
        trace!("result: {:?}", message);
        match message.expect("Message decoding failed") {
            Message::SensorFrame(ref frame) => {
                assert_eq!(0 as ::std::os::raw::c_int, frame.object_detections.list.count)
            },
            _ => panic!("Wrong message variant")
        }
    }

    #[test]
    fn encode_update_subscription() {
        init_logger();
        let subscription = Message::UpdateSubscription(unsafe {
            let mut subscription = Box::new(mem::zeroed::<UpdateSubscription>());
            subscription.subscription_status = SubscriptionStatus_SubscriptionStatus_subscribed as SubscriptionStatus_t;
            subscription.message_period = ptr::null_mut();
            subscription
        });
        test_encode(subscription, &[0x40, 0x00]);
    }

    #[test]
    fn decode_update_subscription() {
        init_logger();
        let message = Message::decode_update_subscription(&[0x40, 0x00]);
        trace!("result: {:?}", message);
        match message.expect("Message decoding failed") {
            Message::UpdateSubscription(ref sub) => {
                assert_eq!(SubscriptionStatus_SubscriptionStatus_subscribed as SubscriptionStatus_t, sub.subscription_status);
                assert_eq!(ptr::null_mut(), sub.message_period);
            },
            _ => panic!("Wrong message variant")
        }
    }

    #[test]
    fn encode_client_registration() {
        init_logger();
        let registration = Message::Registration(unsafe {
            let mut registration = Box::new(mem::zeroed::<ClientRegistration>());
            registration.type_ = ClientType_ClientType_vehicle as ClientType_t;
            registration.covered_area           = ptr::null_mut();
            registration.minimum_message_period = ptr::null_mut();
            registration
        });
        test_encode(registration, &[0x20, 0x00, 0x00])
    }

    #[test]
    fn decode_client_registration() {
        init_logger();
        let message = Message::decode_client_registration(&[0x20, 0x00, 0x00]);
        trace!("result: {:?}", message);
        assert!(message.is_ok());
        match message.expect("Message decoding failed") {
            Message::Registration(ref reg) => {
                assert_eq!(ClientType_ClientType_vehicle as ClientType_t, reg.type_);
                assert_eq!(ptr::null_mut(), reg.covered_area);
                assert_eq!(ptr::null_mut(), reg.minimum_message_period);
            },
            _ => panic!("Wrong message variant")
        }
    }
}