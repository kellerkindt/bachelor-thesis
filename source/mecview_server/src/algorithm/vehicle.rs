use std::fmt::Debug;

pub trait Vehicle<I: Debug + Send, E: Debug + Send> {

    fn init(&mut self, initializer: I);

    fn update(&mut self, environment_model: E);
}