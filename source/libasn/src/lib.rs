#[macro_use]
extern crate log;
#[cfg(test)]
extern crate log4rs;
pub extern crate libasn_sys as raw;
extern crate libmessages;

use libmessages::RawMessage;

mod uper;
mod xer;


const TYPE_ID_REGISTRATION          : u32 = 1;
const TYPE_ID_SENSOR_FRAME          : u32 = 2;
const TYPE_ID_ENVIRONMENT_FRAME     : u32 = 3;
const TYPE_ID_UPDATE_SUBSCRIPTION   : u32 = 4;
const TYPE_ID_INIT_MESSAGE          : u32 = 5;
const TYPE_ID_ROAD_CLEARANCE_FRAME  : u32 = 6;
const TYPE_ID_SENSOR_IDLE_FRAME     : u32 = 7;
const TYPE_ID_UPDATE_STATUS         : u32 = 8;

#[derive(Debug)]
pub enum Message {
    ClientRegistration(Box<raw::ClientRegistration>),
    UpdateSubscription(Box<raw::UpdateSubscription>),
    SensorFrame(Box<raw::SensorFrame>),
    EnvironmentFrame(Box<raw::EnvironmentFrame>),
    RoadClearanceFrame(Box<raw::RoadClearanceFrame>),
    SensorIdleFrame(Box<raw::SensorIdleFrame>),
    UpdateStatus(Box<raw::UpdateStatus>),
    InitMessage(Box<raw::InitMessage>),
}

impl Message {
    pub fn try_decode_uper_client_registration(buffer: &[u8]) -> Result<Message, ()> {
        Ok(Message::ClientRegistration(
            <raw::ClientRegistration as AsnMessage>::try_decode_uper_from_buffer(buffer)?,
        ))
    }

    pub fn try_decode_xer_client_registration(xml: &str) -> Result<Message, ()> {
        Ok(Message::ClientRegistration(
            <raw::ClientRegistration as AsnMessage>::try_decode_xer(xml)?,
        ))
    }

    pub fn try_decode_uper_update_subscription(buffer: &[u8]) -> Result<Message, ()> {
        Ok(Message::UpdateSubscription(
            <raw::UpdateSubscription as AsnMessage>::try_decode_uper_from_buffer(buffer)?,
        ))
    }

    pub fn try_decode_xer_update_subscription(xml: &str) -> Result<Message, ()> {
        Ok(Message::UpdateSubscription(
            <raw::UpdateSubscription as AsnMessage>::try_decode_xer(xml)?,
        ))
    }

    pub fn try_decode_uper_sensor_frame(buffer: &[u8]) -> Result<Message, ()> {
        Ok(Message::SensorFrame(
            <raw::SensorFrame as AsnMessage>::try_decode_uper_from_buffer(buffer)?,
        ))
    }

    pub fn try_decode_xer_sensor_frame(xml: &str) -> Result<Message, ()> {
        Ok(Message::SensorFrame(
            <raw::SensorFrame as AsnMessage>::try_decode_xer(xml)?,
        ))
    }

    pub fn try_decode_uper_environment_frame(buffer: &[u8]) -> Result<Message, ()> {
        Ok(Message::EnvironmentFrame(
            <raw::EnvironmentFrame as AsnMessage>::try_decode_uper_from_buffer(buffer)?,
        ))
    }

    pub fn try_decode_xer_environment_frame(xml: &str) -> Result<Message, ()> {
        Ok(Message::EnvironmentFrame(
            <raw::EnvironmentFrame as AsnMessage>::try_decode_xer(xml)?,
        ))
    }

    pub fn try_decode_uper_road_clearance_frame(buffer: &[u8]) -> Result<Message, ()> {
        Ok(Message::RoadClearanceFrame(
            <raw::RoadClearanceFrame as AsnMessage>::try_decode_uper_from_buffer(buffer)?,
        ))
    }

    pub fn try_decode_xer_road_clearance_frame(xml: &str) -> Result<Message, ()> {
        Ok(Message::RoadClearanceFrame(
            <raw::RoadClearanceFrame as AsnMessage>::try_decode_xer(xml)?,
        ))
    }

    pub fn try_decode_uper_sensor_idle_frame(buffer: &[u8]) -> Result<Message, ()> {
        Ok(Message::SensorIdleFrame(
            <raw::SensorIdleFrame as AsnMessage>::try_decode_uper_from_buffer(buffer)?,
        ))
    }

    pub fn try_decode_xer_sensor_idle_frame(xml: &str) -> Result<Message, ()> {
        Ok(Message::SensorIdleFrame(
            <raw::SensorIdleFrame as AsnMessage>::try_decode_xer(xml)?,
        ))
    }

    pub fn try_decode_uper_update_status(buffer: &[u8]) -> Result<Message, ()> {
        Ok(Message::UpdateStatus(
            <raw::UpdateStatus as AsnMessage>::try_decode_uper_from_buffer(buffer)?,
        ))
    }

    pub fn try_decode_xer_update_status(xml: &str) -> Result<Message, ()> {
        Ok(Message::UpdateStatus(
            <raw::UpdateStatus as AsnMessage>::try_decode_xer(xml)?,
        ))
    }

    pub fn try_decode_uper_init_message(buffer: &[u8]) -> Result<Message, ()> {
        Ok(Message::InitMessage(
            <raw::InitMessage as AsnMessage>::try_decode_uper_from_buffer(buffer)?,
        ))
    }

    pub fn try_decode_xer_init_message(xml: &str) -> Result<Message, ()> {
        Ok(Message::InitMessage(
            <raw::InitMessage as AsnMessage>::try_decode_xer(xml)?,
        ))
    }

    pub fn type_id(&self) -> u32 {
        match self {
            Message::ClientRegistration(_)  => <raw::ClientRegistration as AsnMessage>::type_id(),
            Message::SensorFrame(_)         => <raw::SensorFrame        as AsnMessage>::type_id(),
            Message::EnvironmentFrame(_)    => <raw::EnvironmentFrame   as AsnMessage>::type_id(),
            Message::UpdateSubscription(_)  => <raw::UpdateSubscription as AsnMessage>::type_id(),
            Message::InitMessage(_)         => <raw::InitMessage        as AsnMessage>::type_id(),
            Message::RoadClearanceFrame(_)  => <raw::RoadClearanceFrame as AsnMessage>::type_id(),
            Message::SensorIdleFrame(_)     => <raw::SensorIdleFrame    as AsnMessage>::type_id(),
            Message::UpdateStatus(_)        => <raw::UpdateStatus       as AsnMessage>::type_id(),
        }
    }

    pub fn try_decode_uper(raw: &RawMessage) -> Result<Message, ()> {
        Self::try_decode_uper_from(raw.identifier(), raw.bytes())
    }

    pub fn try_decode_uper_from(type_id: u32, buffer: &[u8]) -> Result<Message, ()> {
        match type_id {
            TYPE_ID_REGISTRATION            => Self::try_decode_uper_client_registration(buffer),
            TYPE_ID_SENSOR_FRAME            => Self::try_decode_uper_sensor_frame(buffer),
            TYPE_ID_ENVIRONMENT_FRAME       => Self::try_decode_uper_environment_frame(buffer),
            TYPE_ID_UPDATE_SUBSCRIPTION     => Self::try_decode_uper_update_subscription(buffer),
            TYPE_ID_INIT_MESSAGE            => Self::try_decode_uper_init_message(buffer),
            TYPE_ID_ROAD_CLEARANCE_FRAME    => Self::try_decode_uper_road_clearance_frame(buffer),
            TYPE_ID_SENSOR_IDLE_FRAME       => Self::try_decode_uper_sensor_idle_frame(buffer),
            TYPE_ID_UPDATE_STATUS           => Self::try_decode_uper_update_status(buffer),
            _ => Err(()),
        }
    }

    pub fn try_encode_uper(self) -> Result<RawMessage, ()> {
        RawMessage::new(self.type_id(), self.try_encode_uper_to_new_buffer()?)
    }

    pub fn try_encode_uper_to(&self, target: &mut [u8]) -> Result<usize, ()> {
        match self {
            Message::ClientRegistration(ref v)  => v.try_encode_uper_to(target),
            Message::UpdateSubscription(ref v)  => v.try_encode_uper_to(target),
            Message::SensorFrame(ref v)         => v.try_encode_uper_to(target),
            Message::EnvironmentFrame(ref v)    => v.try_encode_uper_to(target),
            Message::RoadClearanceFrame(ref v)  => v.try_encode_uper_to(target),
            Message::SensorIdleFrame(ref v)     => v.try_encode_uper_to(target),
            Message::UpdateStatus(ref v)        => v.try_encode_uper_to(target),
            Message::InitMessage(ref v)         => v.try_encode_uper_to(target),
        }
    }

    pub fn try_encode_uper_to_new_buffer(&self) -> Result<Vec<u8>, ()> {
        match self {
            Message::ClientRegistration(v)        => v.try_encode_uper_to_new_buffer(),
            Message::UpdateSubscription(v)  => v.try_encode_uper_to_new_buffer(),
            Message::SensorFrame(v)         => v.try_encode_uper_to_new_buffer(),
            Message::EnvironmentFrame(v)    => v.try_encode_uper_to_new_buffer(),
            Message::RoadClearanceFrame(v)  => v.try_encode_uper_to_new_buffer(),
            Message::SensorIdleFrame(v)     => v.try_encode_uper_to_new_buffer(),
            Message::UpdateStatus(v)        => v.try_encode_uper_to_new_buffer(),
            Message::InitMessage(v)         => v.try_encode_uper_to_new_buffer(),
        }
    }

    pub fn try_encode_xer_to_new_string(&self) -> Result<String, ()> {
        match self {
            Message::ClientRegistration(v)        => v.try_encode_xer_to_new_string(),
            Message::UpdateSubscription(v)  => v.try_encode_xer_to_new_string(),
            Message::SensorFrame(v)         => v.try_encode_xer_to_new_string(),
            Message::EnvironmentFrame(v)    => v.try_encode_xer_to_new_string(),
            Message::RoadClearanceFrame(v)  => v.try_encode_xer_to_new_string(),
            Message::SensorIdleFrame(v)     => v.try_encode_xer_to_new_string(),
            Message::UpdateStatus(v)        => v.try_encode_xer_to_new_string(),
            Message::InitMessage(v)         => v.try_encode_xer_to_new_string(),
        }
    }
}

pub trait AsnMessage: Default {
    fn type_id() -> u32;

    fn type_def() -> &'static mut raw::asn_TYPE_descriptor_t;

    fn try_encode_uper_to(&self, buffer: &mut [u8]) -> Result<usize, ()>
        where
            Self: Sized,
    {
        unsafe { uper::encode(Self::type_def(), self, buffer) }
    }

    fn try_encode_uper_to_new_buffer(&self) -> Result<Vec<u8>, ()>
        where
            Self: Sized,
    {
        uper::encode_to_new_buffer(Self::type_def(), self)
    }

    fn try_encode_uper(&self) -> Result<RawMessage, ()>
        where
            Self: Sized,
    {
        RawMessage::new(Self::type_id(), self.try_encode_uper_to_new_buffer()?)
    }

    fn try_decode_uper_from_buffer(buffer: &[u8]) -> Result<Box<Self>, ()>
        where
            Self: Sized,
    {
        unsafe { uper::decode(Self::type_def(), buffer) }
    }

    fn try_decode_uper(raw: &RawMessage) -> Result<Box<Self>, ()>
        where
            Self: Sized,
    {
        Self::try_decode_uper_from_buffer(raw.bytes())
    }

    fn try_encode_xer_to_new_string(&self) -> Result<String, ()>
        where
            Self: Sized,
    {
        xer::encode_to_new_string(Self::type_def(), self)
    }

    fn try_decode_xer(xml: &str) -> Result<Box<Self>, ()>
        where
            Self: Sized,
    {
        xer::decode(Self::type_def(), xml)
    }
}

impl AsnMessage for raw::ClientRegistration {
    fn type_id() -> u32 {
        TYPE_ID_REGISTRATION
    }

    fn type_def() -> &'static mut raw::asn_TYPE_descriptor_t {
        unsafe { &mut raw::asn_DEF_ClientRegistration }
    }
}

impl AsnMessage for raw::SensorFrame {
    fn type_id() -> u32 {
        TYPE_ID_SENSOR_FRAME
    }

    fn type_def() -> &'static mut raw::asn_TYPE_descriptor_t {
        unsafe { &mut raw::asn_DEF_SensorFrame }
    }
}

impl AsnMessage for raw::EnvironmentFrame {
    fn type_id() -> u32 {
        TYPE_ID_ENVIRONMENT_FRAME
    }

    fn type_def() -> &'static mut raw::asn_TYPE_descriptor_t {
        unsafe { &mut raw::asn_DEF_EnvironmentFrame }
    }
}

impl AsnMessage for raw::UpdateSubscription {
    fn type_id() -> u32 {
        TYPE_ID_UPDATE_SUBSCRIPTION
    }

    fn type_def() -> &'static mut raw::asn_TYPE_descriptor_t {
        unsafe { &mut raw::asn_DEF_UpdateSubscription }
    }
}

impl AsnMessage for raw::InitMessage {
    fn type_id() -> u32 {
        TYPE_ID_INIT_MESSAGE
    }

    fn type_def() -> &'static mut raw::asn_TYPE_descriptor_t {
        unsafe { &mut raw::asn_DEF_InitMessage }
    }
}

impl AsnMessage for raw::RoadClearanceFrame {
    fn type_id() -> u32 {
        TYPE_ID_ROAD_CLEARANCE_FRAME
    }

    fn type_def() -> &'static mut raw::asn_TYPE_descriptor_t {
        unsafe { &mut raw::asn_DEF_RoadClearanceFrame }
    }
}

impl AsnMessage for raw::SensorIdleFrame {
    fn type_id() -> u32 {
        TYPE_ID_SENSOR_IDLE_FRAME
    }

    fn type_def() -> &'static mut raw::asn_TYPE_descriptor_t {
        unsafe { &mut raw::asn_DEF_SensorIdleFrame }
    }
}

impl AsnMessage for raw::UpdateStatus {
    fn type_id() -> u32 {
        TYPE_ID_UPDATE_STATUS
    }

    fn type_def() -> &'static mut raw::asn_TYPE_descriptor_t {
        unsafe { &mut raw::asn_DEF_UpdateStatus }
    }
}

#[cfg(test)]
mod tests {
    use super::raw::*;
    use super::*;
    use std::mem;
    use std::ptr;

    const XML_INIT_MESSAGE:         &'static str = include_str!("test_init_message.xml");
    const XML_UPDATE_STATUS:        &'static str = include_str!("test_update_status.xml");
    const XML_SENSOR_IDLE_FRAME:    &'static str = include_str!("test_sensor_idle_frame.xml");
    const XML_ROAD_CLEARANCE_FRAME: &'static str = include_str!("test_road_clearance_frame.xml");
    const XML_ENVIRONMENT_FRAME:    &'static str = include_str!("test_environment_frame.xml");
    const XML_SENSOR_FRAME:         &'static str = include_str!("test_sensor_frame.xml");
    const XML_UPDATE_SUBSCRIPTION:  &'static str = include_str!("test_update_subscription.xml");
    const XML_CLIENT_REGISTRATION:  &'static str = include_str!("test_client_registration.xml");

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

    fn test_encode_uper(message: Message, should_be: &[u8]) {
        let mut buffer = vec![0u8; should_be.len()];
        let result = message.try_encode_uper_to(&mut buffer[..]);
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

    fn test_encode_xer(message: Message, should_be: &str) {
        let result = message.try_encode_xer_to_new_string();
        trace!("result {:?}", result);
        assert!(result.is_ok());
        let result = result.unwrap();
        warn!("{}", result);
        assert_eq!(&result, should_be);
    }

    fn create_init_message() -> Message {
        Message::InitMessage(unsafe {
            let mut init = Box::new(mem::zeroed::<InitMessage>());
            init.header.timestamp = 400;
            init.envelope.version = 2;
            init.envelope.server_id = 40;
            init.envelope.reference_point.latitude = 484010822;
            init.envelope.reference_point.longitude = 99876076;
            init.envelope.reference_point.altitude = 256000;
            raw::asn_set_empty(
                &mut init.envelope.error_codes as *mut _ as *mut ::std::os::raw::c_void,
            );
            raw::asn_set_empty(&mut init.sectors as *mut _ as *mut ::std::os::raw::c_void);
            init
        })
    }

    fn check_init_message(message: Result<Message, ()>) {
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
            }
            _ => panic!("Wrong message variant"),
        }
    }

    #[test]
    fn encode_uper_init_message() {
        init_logger();
        test_encode_uper(
            create_init_message(),
            &[
                0x00, 0x00, 0x03, 0x20, 0x02, 0x04, 0x02, 0x51, 0x49, 0xf9, 0x51, 0x19, 0xc4, 0xf7,
                0x3b, 0xb1, 0x25, 0xf2, 0x00, 0x00, 0x00,
            ],
        );
    }

    #[test]
    fn decode_uper_init_message() {
        init_logger();
        let message = Message::try_decode_uper_init_message(&[
            0x00, 0x00, 0x03, 0x20, 0x02, 0x04, 0x02, 0x51, 0x49, 0xf9, 0x51, 0x19, 0xc4, 0xf7,
            0x3b, 0xb1, 0x25, 0xf2, 0x00, 0x00, 0x00,
        ]);
        check_init_message(message);
    }

    #[test]
    fn encode_xer_init_message() {
        init_logger();
        test_encode_xer(create_init_message(), XML_INIT_MESSAGE);
    }

    #[test]
    fn decode_xer_init_message() {
        init_logger();
        check_init_message(Message::try_decode_xer_init_message(XML_INIT_MESSAGE));
    }

    fn create_update_status() -> Message {
        Message::UpdateStatus(unsafe {
            let mut status = Box::new(mem::zeroed::<UpdateStatus>());
            status.ip_address.size = 0;
            status.sensor_status =
                raw::ConnectionStatus_ConnectionStatus_disconnected as ConnectionStatus_t;
            status
        })
    }

    fn check_update_status(message: Result<Message, ()>) {
        match message.expect("Decoding failed") {
            Message::UpdateStatus(ref status) => {
                assert_eq!(0, status.ip_address.size);
                assert_eq!(
                    raw::ConnectionStatus_ConnectionStatus_disconnected as ConnectionStatus_t,
                    status.sensor_status
                );
            }
            _ => panic!("Wrong message variant"),
        }
    }

    #[test]
    fn encode_uper_update_status() {
        init_logger();
        test_encode_uper(create_update_status(), &[0x40, 0x00, 0x40, 0x00]);
    }

    #[test]
    fn decode_uper_update_status() {
        init_logger();
        check_update_status(Message::try_decode_uper_update_status(&[
            0x40, 0x00, 0x40, 0x00,
        ]));
    }

    #[test]
    fn encode_xer_update_status() {
        init_logger();
        test_encode_xer(create_update_status(), XML_UPDATE_STATUS);
    }

    #[test]
    fn decode_xer_update_status() {
        init_logger();
        check_update_status(Message::try_decode_xer_update_status(XML_UPDATE_STATUS));
    }

    fn create_sensor_idle_frame() -> Message {
        Message::SensorIdleFrame(unsafe {
            let mut frame = Box::new(mem::zeroed::<raw::SensorIdleFrame>());
            frame.version = 2;
            frame.pole_id = 3;
            frame.sender_id = 4;
            frame
        })
    }

    fn check_sensor_idle_frame(message: Result<Message, ()>) {
        match message.expect("Decoding failed") {
            Message::SensorIdleFrame(ref frame) => {
                assert_eq!(2, frame.version);
                assert_eq!(3, frame.pole_id);
                assert_eq!(4, frame.sender_id)
            }
            _ => panic!("Wrong message variant"),
        };
    }

    #[test]
    fn encode_uper_sensor_idle_frame() {
        init_logger();
        test_encode_uper(create_sensor_idle_frame(), &[0x02, 0x08, 0x30]);
    }

    #[test]
    fn decode_uper_sensor_idle_frame() {
        init_logger();
        check_sensor_idle_frame(Message::try_decode_uper_sensor_idle_frame(&[
            0x02, 0x08, 0x30,
        ]));
    }

    #[test]
    fn encode_xer_sensor_idle_frame() {
        init_logger();
        test_encode_xer(create_sensor_idle_frame(), XML_SENSOR_IDLE_FRAME);
    }

    #[test]
    fn decode_xer_sensor_idle_frame() {
        init_logger();
        check_sensor_idle_frame(Message::try_decode_xer_sensor_idle_frame(
            XML_SENSOR_IDLE_FRAME,
        ));
    }

    fn create_road_clearance_frame() -> Message {
        Message::RoadClearanceFrame(unsafe {
            let mut frame = Box::new(mem::zeroed::<RoadClearanceFrame>());
            frame.header.timestamp = 300;
            frame.envelope.version = 2;
            frame.envelope.reference_point.latitude = 484010822;
            frame.envelope.reference_point.longitude = 99876076;
            frame.envelope.reference_point.altitude = 256000;
            frame.envelope.server_id = 0;
            raw::asn_set_empty(&mut frame.road_sections as *mut _ as *mut ::std::os::raw::c_void);
            frame
        })
    }

    fn check_road_clearance_frame(message: Result<Message, ()>) {
        match message.expect("Decoding failed") {
            Message::RoadClearanceFrame(ref frame) => {
                assert_eq!(300, frame.header.timestamp);
                assert_eq!(2, frame.envelope.version);
                assert_eq!(484010822, frame.envelope.reference_point.latitude);
                assert_eq!(99876076, frame.envelope.reference_point.longitude);
                assert_eq!(256000, frame.envelope.reference_point.altitude);
                assert_eq!(0, frame.envelope.server_id);
                assert_eq!(0, frame.road_sections.list.count);
            }
            _ => panic!("Wrong message variant"),
        }
    }

    #[test]
    fn encode_uper_road_clearance_frame() {
        init_logger();
        test_encode_uper(
            create_road_clearance_frame(),
            &[
                0x00, 0x00, 0x02, 0x58, 0x02, 0x04, 0x02, 0x01, 0x49, 0xf9, 0x51, 0x19, 0xc4, 0xf7,
                0x3b, 0xb1, 0x25, 0xf2, 0x00, 0x00,
            ],
        );
    }

    #[test]
    fn decode_uper_road_clearance_frame() {
        init_logger();
        check_road_clearance_frame(Message::try_decode_uper_road_clearance_frame(&[
            0x00, 0x00, 0x02, 0x58, 0x02, 0x04, 0x02, 0x01, 0x49, 0xf9, 0x51, 0x19, 0xc4, 0xf7,
            0x3b, 0xb1, 0x25, 0xf2, 0x00, 0x00,
        ]));
    }

    #[test]
    fn encode_xer_road_clearance_frame() {
        init_logger();
        test_encode_xer(create_road_clearance_frame(), XML_ROAD_CLEARANCE_FRAME);
    }

    #[test]
    fn decode_xer_road_clearance_frame() {
        init_logger();
        check_road_clearance_frame(Message::try_decode_xer_road_clearance_frame(
            XML_ROAD_CLEARANCE_FRAME,
        ));
    }

    fn create_environment_frame() -> Message {
        Message::EnvironmentFrame(unsafe {
            let mut frame = Box::new(mem::zeroed::<EnvironmentFrame>());
            frame.header.timestamp = 200;
            frame.envelope.version = 1;
            frame.envelope.reference_point.latitude = 484010822;
            frame.envelope.reference_point.longitude = 99876076;
            frame.envelope.reference_point.altitude = 256000;
            frame.envelope.server_id = 0;
            raw::asn_set_empty(
                &mut frame.object_detections as *mut _ as *mut ::std::os::raw::c_void,
            );
            raw::asn_set_empty(
                &mut frame.envelope.error_codes as *mut _ as *mut ::std::os::raw::c_void,
            );
            frame
        })
    }

    fn check_environment_frame(message: Result<Message, ()>) {
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
            }
            _ => panic!("Wrong message variant"),
        }
    }

    #[test]
    fn encode_uper_environment_frame() {
        init_logger();
        test_encode_uper(
            create_environment_frame(),
            &[
                0x00, 0x00, 0x01, 0x90, 0x02, 0x02, 0x02, 0x01, 0x49, 0xf9, 0x51, 0x19, 0xc4, 0xf7,
                0x3b, 0xb1, 0x25, 0xf2, 0x00, 0x00, 0x00,
            ],
        );
    }

    #[test]
    fn decode_uper_environment_frame() {
        init_logger();
        check_environment_frame(Message::try_decode_uper_environment_frame(&[
            0x00, 0x00, 0x01, 0x90, 0x02, 0x02, 0x02, 0x01, 0x49, 0xf9, 0x51, 0x19, 0xc4, 0xf7,
            0x3b, 0xb1, 0x25, 0xf2, 0x00, 0x00, 0x00,
        ]));
    }

    #[test]
    fn encode_xer_environment_frame() {
        init_logger();
        test_encode_xer(create_environment_frame(), XML_ENVIRONMENT_FRAME);
    }

    #[test]
    fn decode_xer_environment_frame() {
        init_logger();
        check_environment_frame(Message::try_decode_xer_environment_frame(
            XML_ENVIRONMENT_FRAME,
        ))
    }

    fn create_sensor_frame() -> Message {
        Message::SensorFrame(unsafe {
            let mut frame = Box::new(mem::zeroed::<SensorFrame>());
            raw::asn_set_empty(
                &mut frame.object_detections as *mut _ as *mut ::std::os::raw::c_void,
            );
            frame.header.timestamp = 100;
            frame.envelope.version = 1;
            frame.envelope.sensor_type = raw::SensorType_SensorType_lidar as SensorType_t;
            frame.envelope.sender_id = 0;
            frame.envelope.pole_id = 2;
            frame.envelope.reference_point.latitude = 484010822;
            frame.envelope.reference_point.longitude = 99876076;
            frame.envelope.reference_point.altitude = 256000;
            frame
        })
    }

    fn check_sensor_frame(message: Result<Message, ()>) {
        match message.expect("Message decoding failed") {
            Message::SensorFrame(ref frame) => {
                assert_eq!(
                    0 as ::std::os::raw::c_int,
                    frame.object_detections.list.count
                );
                assert_eq!(100, frame.header.timestamp);
                assert_eq!(1, frame.envelope.version);
                assert_eq!(
                    raw::SensorType_SensorType_lidar as SensorType_t,
                    frame.envelope.sensor_type
                );
                assert_eq!(0, frame.envelope.sender_id);
                assert_eq!(2, frame.envelope.pole_id);
                assert_eq!(484010822, frame.envelope.reference_point.latitude);
                assert_eq!(99876076, frame.envelope.reference_point.longitude);
                assert_eq!(256000, frame.envelope.reference_point.altitude);
            }
            _ => panic!("Wrong message variant"),
        }
    }

    #[test]
    fn encode_uper_sensor_frame() {
        init_logger();
        test_encode_uper(
            create_sensor_frame(),
            &[
                0x00, 0x00, 0x00, 0xc8, 0x02, 0x00, 0x15, 0x27, 0xe5, 0x44, 0x67, 0x13, 0xdc, 0xee,
                0xc4, 0x97, 0xc8, 0x00, 0x00,
            ],
        );
    }

    #[test]
    fn decode_uper_sensor_frame() {
        init_logger();
        check_sensor_frame(Message::try_decode_uper_sensor_frame(&[
            0x00, 0x00, 0x00, 0xc8, 0x02, 0x00, 0x15, 0x27, 0xe5, 0x44, 0x67, 0x13, 0xdc, 0xee,
            0xc4, 0x97, 0xc8, 0x00, 0x00,
        ]));
    }

    #[test]
    fn encode_xer_sensor_frame() {
        init_logger();
        test_encode_xer(create_sensor_frame(), XML_SENSOR_FRAME)
    }

    #[test]
    fn decode_xer_sensor_frame() {
        init_logger();
        check_sensor_frame(Message::try_decode_xer_sensor_frame(XML_SENSOR_FRAME))
    }

    fn create_update_subscription() -> Message {
        Message::UpdateSubscription(unsafe {
            let mut subscription = Box::new(mem::zeroed::<UpdateSubscription>());
            subscription.subscription_status =
                SubscriptionStatus_SubscriptionStatus_subscribed as SubscriptionStatus_t;
            subscription.message_period = ptr::null_mut();
            subscription
        })
    }

    fn check_update_subscription(message: Result<Message, ()>) {
        match message.expect("Message decoding failed") {
            Message::UpdateSubscription(ref sub) => {
                assert_eq!(
                    SubscriptionStatus_SubscriptionStatus_subscribed as SubscriptionStatus_t,
                    sub.subscription_status
                );
                assert_eq!(ptr::null_mut(), sub.message_period);
            }
            _ => panic!("Wrong message variant"),
        }
    }

    #[test]
    fn encode_uper_update_subscription() {
        init_logger();
        test_encode_uper(create_update_subscription(), &[0x40]);
    }

    #[test]
    fn decode_uper_update_subscription() {
        init_logger();
        check_update_subscription(Message::try_decode_uper_update_subscription(&[0x40]))
    }

    #[test]
    fn encode_xer_update_subscription() {
        init_logger();
        test_encode_xer(create_update_subscription(), XML_UPDATE_SUBSCRIPTION);
    }

    #[test]
    fn decode_xer_update_subscription() {
        init_logger();
        check_update_subscription(Message::try_decode_xer_update_subscription(
            XML_UPDATE_SUBSCRIPTION,
        ));
    }

    fn create_client_registration() -> Message {
        Message::ClientRegistration(unsafe {
            let mut registration = Box::new(mem::zeroed::<ClientRegistration>());
            registration.type_ = ClientType_ClientType_vehicle as ClientType_t;
            registration.covered_area = ptr::null_mut();
            registration.minimum_message_period = ptr::null_mut();
            registration
        })
    }

    fn check_client_registration(message: Result<Message, ()>) {
        match message.expect("Message decoding failed") {
            Message::ClientRegistration(ref reg) => {
                assert_eq!(ClientType_ClientType_vehicle, reg.type_ as ClientType);
                assert_eq!(ptr::null_mut(), reg.covered_area);
                assert_eq!(ptr::null_mut(), reg.minimum_message_period);
            }
            _ => panic!("Wrong message variant"),
        }
    }

    #[test]
    fn encode_uper_client_registration() {
        init_logger();
        test_encode_uper(create_client_registration(), &[0x20])
    }

    #[test]
    fn decode_uper_client_registration() {
        init_logger();
        check_client_registration(Message::try_decode_uper_client_registration(&[0x20]));
    }

    #[test]
    fn encode_xer_client_registration() {
        init_logger();
        test_encode_xer(create_client_registration(), XML_CLIENT_REGISTRATION);
    }

    #[test]
    fn decode_xer_client_registration() {
        init_logger();
        check_client_registration(Message::try_decode_xer_client_registration(
            XML_CLIENT_REGISTRATION,
        ));
    }
}
