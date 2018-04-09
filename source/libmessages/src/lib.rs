pub mod asn;

#[macro_use]
extern crate log;

#[cfg(test)]
extern crate log4rs;


pub struct RawMessage<T: ?Sized> {
    identifier: u32,
    bytes: Vec<u8>,
    _t: ::std::marker::PhantomData<T>
}

impl<T: ?Sized> RawMessage<T> {
    pub(crate) fn new(identifier: u32, bytes: Vec<u8>) -> Result<RawMessage<T>, ()> {
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
}