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
    EnvironmentFrame(Box<raw::EnvironmentFrame>),
    RoadClearanceFrame(Box<raw::RoadClearanceFrame>),
    SensorIdleFrame(Box<raw::SensorIdleFrame>),
    UpdateStatus(Box<raw::UpdateStatus>),
    InitMessage(Box<raw::InitMessage>),
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

    pub fn decode_environment_frame(buffer: &[u8]) -> Result<Message, ()> {
        Ok(Message::EnvironmentFrame(unsafe {
            asn::uper_decode(&mut raw::asn_DEF_EnvironmentFrame, buffer)?
        }))
    }

    pub fn decode_road_clearance_frame(buffer: &[u8]) -> Result<Message, ()> {
        Ok(Message::RoadClearanceFrame(unsafe {
            asn::uper_decode(&mut raw::asn_DEF_RoadClearanceFrame, buffer)?
        }))
    }

    pub fn decode_sensor_idle_frame(buffer: &[u8]) -> Result<Message, ()> {
        Ok(Message::SensorIdleFrame(unsafe {
            asn::uper_decode(&mut raw::asn_DEF_SensorIdleFrame, buffer)?
        }))
    }

    pub fn decode_update_status(buffer: &[u8]) -> Result<Message, ()> {
        Ok(Message::UpdateStatus(unsafe {
            asn::uper_decode(&mut raw::asn_DEF_UpdateStatus, buffer)?
        }))
    }

    pub fn decode_init_message(buffer: &[u8]) -> Result<Message, ()> {
        Ok(Message::InitMessage(unsafe {
            asn::uper_decode(&mut raw::asn_DEF_InitMessage, buffer)?
        }))
    }

    pub fn type_id(&self) -> u32 {
        match self {
            Message::Registration(_)        => 1,
            Message::SensorFrame(_)         => 2,
            Message::EnvironmentFrame(_)    => 3,
            Message::UpdateSubscription(_)  => 4,
            Message::InitMessage(_)         => 5,
            Message::RoadClearanceFrame(_)  => 6,
            Message::SensorIdleFrame(_)     => 7,
            Message::UpdateStatus(_)        => 8,
        }
    }

    pub fn decode(type_id: u32, buffer: &[u8]) -> Result<Message, ()> {
        match type_id {
            1 => Self::decode_client_registration(buffer),
            2 => Self::decode_sensor_frame(buffer),
            3 => Self::decode_environment_frame(buffer),
            4 => Self::decode_update_subscription(buffer),
            5 => Self::decode_init_message(buffer),
            6 => Self::decode_road_clearance_frame(buffer),
            7 => Self::decode_sensor_idle_frame(buffer),
            8 => Self::decode_update_status(buffer),
            _ => Err(())
        }
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
            },
            Message::EnvironmentFrame(ref frame) => unsafe {
                asn::uper_encode(&mut raw::asn_DEF_EnvironmentFrame, frame.as_ref(), target)
            },
            Message::RoadClearanceFrame(ref frame) => unsafe {
                asn::uper_encode(&mut raw::asn_DEF_RoadClearanceFrame, frame.as_ref(), target)
            },
            Message::SensorIdleFrame(ref frame) => unsafe {
                asn::uper_encode(&mut raw::asn_DEF_SensorIdleFrame, frame.as_ref(), target)
            },
            Message::UpdateStatus(ref status) => unsafe {
                asn::uper_encode(&mut raw::asn_DEF_UpdateStatus, status.as_ref(), target)
            },
            Message::InitMessage(ref status) => unsafe {
                asn::uper_encode(&mut raw::asn_DEF_InitMessage, status.as_ref(), target)
            },
        }
    }
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe {
            match self {
                // free only content, since the box will free the struct itself
                Message::Registration(ref value) => raw::free_content(&mut raw::asn_DEF_ClientRegistration, value.as_ref()),
                Message::UpdateSubscription(ref value) => raw::free_content(&mut raw::asn_DEF_UpdateSubscription, value.as_ref()),
                Message::SensorFrame(ref value) => raw::free_content(&mut raw::asn_DEF_SensorFrame, value.as_ref()),
                Message::EnvironmentFrame(ref value) => raw::free_content(&mut raw::asn_DEF_EnvironmentFrame, value.as_ref()),
                Message::RoadClearanceFrame(ref value) => raw::free_content(&mut raw::asn_DEF_RoadClearanceFrame, value.as_ref()),
                Message::SensorIdleFrame(ref value) => raw::free_content(&mut raw::asn_DEF_SensorIdleFrame, value.as_ref()),
                Message::UpdateStatus(ref status) => raw::free_content(&mut raw::asn_DEF_UpdateStatus, status.as_ref()),
                Message::InitMessage(ref init) => raw::free_content(&mut raw::asn_DEF_InitMessage, init.as_ref()),
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
            .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
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
                string.push_str(&format!("0x{:02x}, ", byte));
            }
            debug!("{}", string);
        }
        assert_eq!(Ok(should_be.len()), result);
        assert_eq!(should_be, &buffer[..]);
    }

    #[test]
    fn encode_init_message() {
        init_logger();
        let init = Message::InitMessage(unsafe {
            let mut init = Box::new(mem::zeroed::<InitMessage>());
            init.header.timestamp = 400;
            init.envelope.version = 2;
            init.envelope.server_id = 40;
            init.envelope.reference_point.latitude = 484010822;
            init.envelope.reference_point.longitude = 99876076;
            init.envelope.reference_point.altitude = 256000;
            raw::asn_set_empty(&mut init.envelope.error_codes as *mut _ as *mut ::std::os::raw::c_void);
            raw::asn_set_empty(&mut init.sectors as *mut _ as *mut ::std::os::raw::c_void);
            init
        });
        test_encode(init, &[
            0x00, 0x00, 0x03, 0x20, 0x02, 0x04, 0x02, 0x51, 0x49, 0xf9, 0x51, 0x19, 0xc4, 0xf7, 0x3b,
            0xb1, 0x25, 0xf2, 0x00, 0x00, 0x00,
        ]);
    }

    #[test]
    fn decode_init_message() {
        init_logger();
        let message = Message::decode_init_message(&[
            0x00, 0x00, 0x03, 0x20, 0x02, 0x04, 0x02, 0x51, 0x49, 0xf9, 0x51, 0x19, 0xc4, 0xf7, 0x3b,
            0xb1, 0x25, 0xf2, 0x00, 0x00, 0x00,
        ]);
        match message.expect("Decoding failed") {
            Message::InitMessage(ref init) => {
                assert_eq!(400, init.header.timestamp);
                assert_eq!(2, init.envelope.version);
                assert_eq!(40, init.envelope.server_id);
                assert_eq!(484010822, init.envelope.reference_point.latitude);
                assert_eq!(99876076, init.envelope.reference_point.longitude);
                assert_eq!(256000, init.envelope.reference_point.altitude);
                assert_eq!(0, init.envelope.error_codes.list.count);
                assert_eq!(0, init.sectors.list.count);
            },
            _ => panic!("Wrong message variant")
        }
    }

    #[test]
    fn encode_update_status() {
        init_logger();
        let status = Message::UpdateStatus(unsafe {
            let mut status = Box::new(mem::zeroed::<UpdateStatus>());
            status.ip_address.size = 0;
            status.sensor_status = raw::ConnectionStatus_ConnectionStatus_disconnected as ConnectionStatus_t;
            status
        });
        test_encode(status, &[
            0x40, 0x00, 0x40, 0x00,
        ]);
    }

    #[test]
    fn decode_update_status() {
        init_logger();
        let message = Message::decode_update_status(&[
            0x40, 0x00, 0x40, 0x00,
        ]);
        match message.expect("Decoding failed") {
            Message::UpdateStatus(ref status) => {
                assert_eq!(0, status.ip_address.size);
                assert_eq!(raw::ConnectionStatus_ConnectionStatus_disconnected as ConnectionStatus_t, status.sensor_status);
            },
            _ => panic!("Wrong message variant")
        }
    }

    #[test]
    fn encode_sensor_idle_frame() {
        init_logger();
        let frame = Message::SensorIdleFrame(unsafe {
            let mut frame = Box::new(mem::zeroed::<raw::SensorIdleFrame>());
            frame.version = 2;
            frame.pole_id = 3;
            frame.sender_id = 4;
            frame
        });
        test_encode(frame, &[
            0x02, 0x08, 0x30
        ]);
    }

    #[test]
    fn decode_sensor_idle_frame() {
        init_logger();
        let message = Message::decode_sensor_idle_frame(&[
            0x02, 0x08, 0x30
        ]);
        match message.expect("Decoding failed") {
            Message::SensorIdleFrame(ref frame) => {
                assert_eq!(2, frame.version);
                assert_eq!(3, frame.pole_id);
                assert_eq!(4, frame.sender_id);
            },
            _ => panic!("Wrong message variant")
        }
    }

    #[test]
    fn encode_road_clearance_frame() {
        init_logger();
        let frame = Message::RoadClearanceFrame(unsafe {
            let mut frame = Box::new(mem::zeroed::<RoadClearanceFrame>());
            frame.header.timestamp = 300;
            frame.envelope.version = 2;
            frame.envelope.reference_point.latitude = 484010822;
            frame.envelope.reference_point.longitude = 99876076;
            frame.envelope.reference_point.altitude = 256000;
            frame.envelope.server_id = 0;
            raw::asn_set_empty(&mut frame.road_sections as *mut _ as *mut ::std::os::raw::c_void);
            frame
        });
        test_encode(frame, &[
            0x00, 0x00, 0x02, 0x58, 0x02, 0x04, 0x02, 0x01, 0x49, 0xf9, 0x51, 0x19, 0xc4, 0xf7, 0x3b,
            0xb1, 0x25, 0xf2, 0x00, 0x00,
        ]);
    }

    #[test]
    fn decode_road_clearance_frame() {
        init_logger();
        let message = Message::decode_road_clearance_frame(&[
            0x00, 0x00, 0x02, 0x58, 0x02, 0x04, 0x02, 0x01, 0x49, 0xf9, 0x51, 0x19, 0xc4, 0xf7, 0x3b,
            0xb1, 0x25, 0xf2, 0x00, 0x00,
        ]);
        match message.expect("Decoding failed") {
            Message::RoadClearanceFrame(ref frame) => {
                assert_eq!(300, frame.header.timestamp);
                assert_eq!(2, frame.envelope.version);
                assert_eq!(484010822, frame.envelope.reference_point.latitude);
                assert_eq!(99876076, frame.envelope.reference_point.longitude);
                assert_eq!(256000, frame.envelope.reference_point.altitude);
                assert_eq!(0, frame.envelope.server_id);
                assert_eq!(0, frame.road_sections.list.count);
            },
            _ => panic!("Wrong message variant")
        }
    }

    #[test]
    fn encode_environment_frame() {
        init_logger();
        let frame = Message::EnvironmentFrame(unsafe {
            let mut frame = Box::new(mem::zeroed::<EnvironmentFrame>());
            frame.header.timestamp = 200;
            frame.envelope.version = 1;
            frame.envelope.reference_point.latitude = 484010822;
            frame.envelope.reference_point.longitude = 99876076;
            frame.envelope.reference_point.altitude = 256000;
            frame.envelope.server_id = 0;
            raw::asn_set_empty(&mut frame.object_detections as *mut _ as *mut ::std::os::raw::c_void);
            raw::asn_set_empty(&mut frame.envelope.error_codes as *mut _ as *mut ::std::os::raw::c_void);
            frame
        });
        test_encode(frame, &[
            0x00, 0x00, 0x01, 0x90, 0x02, 0x02, 0x02, 0x01, 0x49, 0xf9, 0x51, 0x19, 0xc4, 0xf7, 0x3b,
            0xb1, 0x25, 0xf2, 0x00, 0x00, 0x00,
        ]);
    }

    #[test]
    fn decode_environment_frame() {
        init_logger();
        let message = Message::decode_environment_frame(&[
            0x00, 0x00, 0x01, 0x90, 0x02, 0x02, 0x02, 0x01, 0x49, 0xf9, 0x51, 0x19, 0xc4, 0xf7, 0x3b,
            0xb1, 0x25, 0xf2, 0x00, 0x00, 0x00,
        ]);
        match message.expect("Decoding failed") {
            Message::EnvironmentFrame(ref frame) => {
                assert_eq!(0, frame.object_detections.list.count);
                assert_eq!(200, frame.header.timestamp);
                assert_eq!(1, frame.envelope.version);
                assert_eq!(484010822, frame.envelope.reference_point.latitude);
                assert_eq!(99876076, frame.envelope.reference_point.longitude);
                assert_eq!(256000, frame.envelope.reference_point.altitude);
                assert_eq!(0, frame.envelope.server_id);
                assert_eq!(0, frame.envelope.error_codes.list.count);

            },
            _ => panic!("Wrong message variant")
        }
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
            0x97, 0xc8, 0x00, 0x00,
        ]);
    }

    #[test]
    fn decode_sensor_frame() {
        init_logger();
        let message = Message::decode_sensor_frame(&[
            0x00, 0x00, 0x00, 0xc8, 0x02, 0x00, 0x15, 0x27, 0xe5, 0x44, 0x67, 0x13, 0xdc, 0xee, 0xc4,
            0x97, 0xc8, 0x00, 0x00,
        ]);
        trace!("result: {:?}", message);
        match message.expect("Message decoding failed") {
            Message::SensorFrame(ref frame) => {
                assert_eq!(0 as ::std::os::raw::c_int, frame.object_detections.list.count);
                assert_eq!(100, frame.header.timestamp);
                assert_eq!(1, frame.envelope.version);
                assert_eq!(raw::SensorType_SensorType_lidar as SensorType_t, frame.envelope.sensor_type);
                assert_eq!(0, frame.envelope.sender_id);
                assert_eq!(2, frame.envelope.pole_id);
                assert_eq!(484010822, frame.envelope.reference_point.latitude);
                assert_eq!(99876076, frame.envelope.reference_point.longitude);
                assert_eq!(256000, frame.envelope.reference_point.altitude);
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
        test_encode(subscription, &[0x40]);
    }

    #[test]
    fn decode_update_subscription() {
        init_logger();
        let message = Message::decode_update_subscription(&[0x40]);
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
        test_encode(registration, &[0x20])
    }

    #[test]
    fn decode_client_registration() {
        init_logger();
        let message = Message::decode_client_registration(&[0x20]);
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