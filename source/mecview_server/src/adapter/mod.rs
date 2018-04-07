
pub mod asn;

use client;
use async::Sender;

#[derive(Debug)]
pub enum Command<M: ::std::fmt::Debug> {
    ProcessMessage(M),
    RemoteSend(M),
    RemoteUnsubscribe,
    RemoteSubscribe,
    RemoteInit,
    Shutdown,
}