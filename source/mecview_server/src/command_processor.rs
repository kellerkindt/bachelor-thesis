use std::io::Error;

pub trait CommandProcessor<T> {
    fn process_command(&mut self, command: T) -> Result<(), Error>;
}
