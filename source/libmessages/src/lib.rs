#[macro_use]
extern crate log;

#[cfg(test)]
extern crate log4rs;
extern crate bytes;

use std::sync::Arc;


#[derive(Debug)]
pub struct RawMessage<T: ?Sized> {
    identifier: u32,
    data: RawMessageData,
    _t: ::std::marker::PhantomData<T>,
}

impl<T: ?Sized> RawMessage<T> {
    pub fn new<I: Into<RawMessageData>>(identifier: u32, data: I) -> Result<RawMessage<T>, ()> {
        let data = data.into();
        if data.len() > ::std::u32::MAX as usize {
            Err(())
        } else {
            Ok(RawMessage {
                identifier,
                data,
                _t: ::std::marker::PhantomData,
            })
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

pub trait Generalize<T> {

}

#[derive(Debug)]
pub enum RawMessageData {
    BytesMut(bytes::BytesMut),
    Vec(Vec<u8>),
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