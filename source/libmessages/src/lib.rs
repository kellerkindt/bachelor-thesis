pub mod asn;

#[macro_use]
extern crate log;

#[cfg(test)]
extern crate log4rs;

use std::sync::Arc;

#[derive(Debug)]
pub struct RawMessage<T: ?Sized> {
    identifier: u32,
    bytes: Vec<u8>,
    _t: ::std::marker::PhantomData<T>,
}

impl<T: ?Sized> RawMessage<T> {
    pub fn new(identifier: u32, bytes: Vec<u8>) -> Result<RawMessage<T>, ()> {
        if bytes.len() > ::std::u32::MAX as usize {
            Err(())
        } else {
            Ok(RawMessage {
                identifier,
                bytes,
                _t: ::std::marker::PhantomData,
            })
        }
    }

    pub fn identifier(&self) -> u32 {
        self.identifier
    }

    pub fn length(&self) -> u32 {
        self.bytes.len() as u32
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..]
    }

    pub(crate) fn into<T2>(self) -> RawMessage<T2> {
        unsafe { ::std::mem::transmute(self) }
    }

    pub(crate) fn arc_into<T2>(myself: Arc<RawMessage<T>>) -> Arc<RawMessage<T2>> {
        unsafe { ::std::mem::transmute(myself) }
    }
}
