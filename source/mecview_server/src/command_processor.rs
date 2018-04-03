

pub trait CommandProcessor<T> {
    fn process_command(&mut self, command: T) -> Result<(), ()>;
}