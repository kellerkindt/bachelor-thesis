#[macro_use]
extern crate log;

use std::fmt::Debug;
use std::io::Error;
use std::ops::IndexMut;
use std::sync::Arc;

use libmessages::RawMessage;

/// Listens for a change of subscriber count, returns an `Error`
/// if this listener is no longer valid
pub type CountListener = Box<FnMut(usize, usize) -> Result<(), Error> + Send + 'static>;

/// Listens for environment model updates, returns an `Error`
/// if this listener is no longer valid
pub type EnvironmentListener<E> =
    Box<FnMut(Arc<RawMessage<E>>) -> Result<(), Error> + Send + 'static>;

pub trait Algorithm {
    /// The message being sent by the sensor to this algorithm
    type SensorMessage: Send + Debug;

    /// The model being calculated by this algorithm
    type EnvironmentModel: Send + Debug;

    /// The identifier used to
    type Identifier: Send + PartialEq + 'static;

    /// Updates the Algorithm with the new `SensorMessage`,
    fn update(&mut self, update: Box<Self::SensorMessage>);

    /// Publishes the given `EnvironmentModel` to all subscribed `EnvironmentListener`s
    fn publish(&mut self, model: RawMessage<Self::EnvironmentModel>);

    /// Adds the given `EnvironmentListener` to the list of listeners
    /// to notify on publications of a new `EnvironmentModel`.
    /// The new `EnvironmentListener` is set as inactive initially
    fn subscribe_environment_model(
        &mut self,
        identifier: Self::Identifier,
        listener: EnvironmentListener<Self::EnvironmentModel>,
    );

    /// Removes the `EnvironmentListener` for the given identifier
    /// and therefore no longer publishes `EnvironmentModel`s to it
    fn unsubscribe_environment_model(&mut self, identifier: Self::Identifier);

    /// Activates the `EnvironmentListener` for the given identifier,
    /// thus notifying it on publications of new `EnvironmentModel`s
    fn activate_environment_model_subscription(&mut self, identifier: Self::Identifier);

    /// Deactivates the `EnvironmentListener` for the given identifier,
    /// thus no longer notifying it on publications of new `EnvironmentModel`s but
    /// not removing the listener completly
    fn deactivate_environment_model_subscription(&mut self, identifier: Self::Identifier);

    /// Adds the given `CountListener` to the list of listeners to notify
    /// on a count change of `EnvironmentListener`s
    fn subscribe_listener_count(&mut self, identifier: Self::Identifier, listener: CountListener);

    /// Remove the `CountListener` for the given identifier from the list of
    /// listeners to notify on a count change of `EnvironmentListener`s
    fn unsubscribe_listener_count(&mut self, identifier: Self::Identifier);
}

/// This struct manages the algorithm organizational
/// overhead of the listeners
#[derive(Default)]
pub struct ListenerManager<S: Send + Debug, E: Send + Debug, I: Send + Debug + PartialEq + 'static>
{
    model_listeners: Vec<(I, bool, EnvironmentListener<E>)>,
    count_listeners: Vec<(I, CountListener)>,
    _s: ::std::marker::PhantomData<S>,
}

impl<S: Send + Debug, E: Send + Debug, I: Send + Debug + PartialEq + 'static>
    ListenerManager<S, E, I>
{
    pub fn add_environment_listener(&mut self, identifier: I, listener: EnvironmentListener<E>) {
        trace!("Adding model listener with id={:?}", identifier);
        let before = self.model_listener.len();
        self.model_listeners.push((identifier, false, listener));

        trace!(
            "Now subscribed model listeners: {}",
            self.model_listeners.len()
        );

        if before != self.model_listeners.len() {
            self.on_model_count_changed(before);
        }
    }

    pub fn remove_environment_listener(&mut self, identifier: I) {
        trace!("Removing model listener with id={:?}", identifier);
        let before = self.model_listeners.len();
        self.model_listeners.retain(|(id, _, _)| id.ne(&remove));
        trace!(
            "Removed {} matches, still subscribed: {}",
            before - self.model_listeners.len(),
            self.model_listeners.len()
        );
        if before != self.model_listener.len() {
            self.on_model_count_changed(before);
        }
    }

    pub fn activate_environment_model_listener(&mut self, identifier: I) {
        self.model_listeners
            .iter_mut()
            .filter(|(id, _, _)| id.eq(&identifier))
            .for_each(|(_, ref mut active, _)| *active = true);
    }

    pub fn deactivate_environment_model_listener(&mut self, identifier: I) {
        self.model_listeners
            .iter_mut()
            .filter(|(id, _, _)| id.eq(&identifier))
            .for_each(|(_, ref mut active, _)| *active = false);
    }

    fn on_model_count_changed(&mut self, before: usize) {
        let after = self.model_listeners.len();
        trace!("on_model_count_changed, before/after: {}/{}", before, after);
        retain_mut(&mut self.count_listeners, |(_, listener)| {
            listener(before, after).is_ok()
        });
    }

    pub fn add_count_listener(&mut self, identifier: I, mut sink: CountListener) {
        trace!("Adding count listener with id={:?}", identifier);
        if sink(0, self.model_listeners.len()).is_ok() {
            self.count_listeners.push((identifier, sink));
        }
        trace!(
            "Now subscribed count listeners: {}",
            self.count_listeners.len()
        );
    }

    pub fn remove_count_listener(&mut self, identifier: I) {
        trace!("Removing count listener with id={:?}", identifier);
        self.count_listeners.retain(|(id, _)| id.ne(&identifier));
        trace!(
            "Still subscribed count listeners. {}",
            self.count_listeners.len()
        );
    }

    pub fn publish_environment_model<M: Into<Arc<RawMessage<E>>>>(&mut self, model: M) {
        let model = model.into();
        let before = self.model_listeners.len();
        retain_mut(&mut self.model_listeners, |(id, active, listener)| {
            trace!("Sending EnvironmentModel to listener with id={:?}", id);
            !*active || listener(model.clone()).is_ok()
        });
        if before != self.model_listeners.len() {
            self.on_model_count_changed(before);
        }
    }
}

pub enum Command<U: Send + Debug, E: Send + Debug, I: Send + Debug + Sized + 'static> {
    Update(Box<U>),
    Publish(RawMessage<E>),
    SubscribeEnvironmentModel(I, EnvironmentListener<E>),
    UnsubscribeEnvironmentModel(I),
    SubscribeListenerCount(I, CountListener),
    UnsubscribeListenerCount(I),
    ActivateEnvironmentModelSubscription(I),
    DeactivateEnvironmentModelSubscription(I),
}

impl<S: Send + Debug, E: Send + Debug, I: Send + Debug + Sized + 'static> Command<S, E, I> {
    pub fn apply(
        self,
        algorithm: &mut Algorithm<SensorMessage = S, EnvironmentModel = E, Identifier = I>,
    ) {
        match self {
            Command::Update(update) => algorithm.update(update),
            Command::Publish(model) => algorithm.publish(model),
            Command::SubscribeEnvironmentModel(identifier, listener) => {
                algorithm.subscribe_environment_model(identifier, listener);
            }
            Command::UnsubscribeEnvironmentModel(identifier) => {
                algorithm.unsubscribe_environment_model(identifier);
            }
            Command::SubscribeListenerCount(identifier, listener) => {
                algorithm.subscribe_listener_count(identifier, listener);
            }
            Command::UnsubscribeListenerCount(identifier) => {
                algorithm.unsubscribe_listener_count(identifier);
            }
            Command::ActivateEnvironmentModelSubscription(identifier) => {
                algorithm.activate_environment_model_subscription(identifier);
            }
            Command::DeactivateEnvironmentModelSubscription(identifier) => {
                algorithm.deactivate_environment_model_subscription(identifier);
            }
        }
    }
}

impl<
        S: Send + Debug,
        E: Send + Debug,
        I: Send + Debug + Sized + 'static,
        F: FnMut(Command<S, E, I>),
    > Algorithm for F
{
    type SensorMessage = S;
    type EnvironmentModel = E;
    type Identifier = I;

    fn update(&mut self, update: Box<<Self as Algorithm>::SensorFrame>) {
        self(Command::Update(update));
    }

    fn publish(&mut self, model: RawMessage<<Self as Algorithm>::EnvironmentModel>) {
        self(Command::Publish(model));
    }

    fn subscribe_environment_model(
        &mut self,
        identifier: <Self as Algorithm>::Identifier,
        listener: EnvironmentListener<<Self as Algorithm>::EnvironmentModel>,
    ) {
        self(Command::SubscribeEnvironmentModel(identifier, listener));
    }

    fn unsubscribe_environment_model(&mut self, identifier: <Self as Algorithm>::Identifier) {
        self(Command::UnsubscribeEnvironmentModel(identifier));
    }

    fn activate_environment_model_subscription(
        &mut self,
        identifier: <Self as Algorithm>::Identifier,
    ) {
        self(Command::ActivateEnvironmentModelSubscription(identifier));
    }

    fn deactivate_environment_model_subscription(
        &mut self,
        identifier: <Self as Algorithm>::Identifier,
    ) {
        self(Command::DeactivateEnvironmentModelSubscription(identifier));
    }

    fn subscribe_listener_count(
        &mut self,
        identifier: <Self as Algorithm>::Identifier,
        listener: CountListener,
    ) {
        self(Command::SubscribeListenerCount(identifier, listener));
    }

    fn unsubscribe_listener_count(&mut self, identifier: <Self as Algorithm>::Identifier) {
        self(Command::UnsubscribeListenerCount(identifier));
    }
}

fn retain_mut<V, F: Fn(&mut V) -> bool>(vec: &mut Vec<V>, keep: F) {
    // nightly: see Vec::drain_filter
    // Vec::retain only provides &T and not &mut T :(
    let mut index = 0_usize;
    for _ in 0..vec.len() {
        if !keep(vec.index_mut(index)) {
            vec.swap_remove(index);
        } else {
            index += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn retain_mut_removes_correct() {
        let mut list: Vec<i32> = vec![1, 2, 3, 4, 5];
        retain_mut(&mut list, |i| *i % 2 == 0);
        assert_eq!(2, list.len());
        assert!(list.contains(&2));
        assert!(list.contains(&4));
    }
}
