extern crate bytes;

#[derive(Debug)]
pub enum RawMessageData {
    BytesMut(bytes::BytesMut),
    Vec(Vec<u8>),
}

#[derive(Debug)]
pub struct RawMessage {
    identifier: u32,
    data: RawMessageData,
}

impl RawMessage {
    pub fn new<I: Into<RawMessageData>>(identifier: u32, data: I) -> Result<RawMessage, ()> {
        let data = data.into();
        if data.len() > ::std::u32::MAX as usize {
            Err(())
        } else {
            Ok(RawMessage { identifier, data })
        }
    }

    pub fn identifier(&self) -> u32 {
        self.identifier
    }

    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn bytes(&self) -> &[u8] {
        self.data.bytes()
    }
}

impl RawMessageData {
    fn len(&self) -> usize {
        match self {
            RawMessageData::BytesMut(bytes) => bytes.len(),
            RawMessageData::Vec(vec) => vec.len(),
        }
    }

    fn bytes(&self) -> &[u8] {
        let len = self.len();
        match self {
            RawMessageData::BytesMut(bytes) => &bytes[..len],
            RawMessageData::Vec(vec) => &vec[..len],
        }
    }
}

impl Into<RawMessageData> for Vec<u8> {
    fn into(self) -> RawMessageData {
        RawMessageData::Vec(self)
    }
}

impl Into<RawMessageData> for bytes::BytesMut {
    fn into(self) -> RawMessageData {
        RawMessageData::BytesMut(self)
    }
}
