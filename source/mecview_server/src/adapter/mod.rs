pub mod asn;

use std::fmt::Debug;
use std::io::Error;

use std::sync::Arc;

use messages::RawMessage;

pub trait Adapter<E: Debug> {
    fn init_vehicle(&mut self) -> Result<(), Error>;

    fn unsubscribe(&mut self) -> Result<(), Error>;

    fn subscribe(&mut self) -> Result<(), Error>;

    fn update_environment_model(&mut self, model: Arc<RawMessage<E>>) -> Result<(), Error>;
}
