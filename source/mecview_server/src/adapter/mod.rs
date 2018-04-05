
pub mod asn;

use client;
use async::Sender;

pub enum Command<M> {
    ProcessMessage(M),
    SendMessage(M),
}