pub mod asn;

use std::io::Error;

use std::sync::Arc;

use libmessages::RawMessage;

pub trait Adapter {
    fn init_vehicle(&mut self) -> Result<(), Error>;

    fn unsubscribe(&mut self) -> Result<(), Error>;

    fn subscribe(&mut self) -> Result<(), Error>;

    fn update_environment_model(&mut self, model: Arc<RawMessage>) -> Result<(), Error>;
}
