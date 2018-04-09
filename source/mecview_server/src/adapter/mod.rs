
pub mod asn;

use std::io::Error;
use std::fmt::Debug;

use client;
use async::Sender;


pub trait Adapter<E: Debug> {

    fn init_vehicle(&mut self) -> Result<(), Error>;

    fn unsubscribe(&mut self) -> Result<(), Error>;

    fn subscribe(&mut self) -> Result<(), Error>;

    fn update_environment_model(&mut self, model: E) -> Result<(), Error>;
}