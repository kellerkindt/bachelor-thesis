pub mod asn;

#[macro_use]
extern crate log;

#[cfg(test)]
extern crate log4rs;


pub struct RawMessage {
    identifier: u32,
    bytes: Vec<u8>,
}

impl RawMessage {
    pub fn new(identifier: u32, bytes: Vec<u8>) -> Result<RawMessage, ()> {
        if bytes.len() > ::std::u32::MAX as usize {
            Err(())
        } else {
            Ok(RawMessage {
                identifier,
                bytes
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