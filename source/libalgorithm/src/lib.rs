#[macro_use]
extern crate log;
extern crate libmessages;

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
pub type EnvironmentListener = Box<FnMut(Arc<RawMessage>) -> Result<(), Error> + Send + 'static>;

/// `S`: The message being sent by the sensor to this algorithm
/// `E`: The model being calculated by this algorithm
/// `I`: The identifier used to
pub trait Algorithm<S: Send + Debug, I: Send + PartialEq + 'static> {
    /// Updates the Algorithm with the new `S`,
    fn update(&mut self, update: Box<S>);

    /// Publishes the given `E` to all subscribed `EnvironmentListener`s
    fn publish(&mut self, model: RawMessage);

    /// Adds the given `EnvironmentListener` to the list of listeners
    /// to notify on publications of a new `E`.
    /// The new `EnvironmentListener` is set as inactive initially
    fn subscribe_environment_model(&mut self, identifier: I, listener: EnvironmentListener);

    /// Removes the `EnvironmentListener` for the given identifier
    /// and therefore no longer publishes `E`s to it
    fn unsubscribe_environment_model(&mut self, identifier: I);

    /// Activates the `EnvironmentListener` for the given identifier,
    /// thus notifying it on publications of new `E`s
    fn activate_environment_model_subscription(&mut self, identifier: I);

    /// Deactivates the `EnvironmentListener` for the given identifier,
    /// thus no longer notifying it on publications of new `E`s but
    /// not removing the listener completly
    fn deactivate_environment_model_subscription(&mut self, identifier: I);

    /// Adds the given `CountListener` to the list of listeners to notify
    /// on a count change of `EnvironmentListener`s
    fn subscribe_listener_count(&mut self, identifier: I, listener: CountListener);

    /// Remove the `CountListener` for the given identifier from the list of
    /// listeners to notify on a count change of `EnvironmentListener`s
    fn unsubscribe_listener_count(&mut self, identifier: I);
}

/// This struct manages the algorithm organizational
/// overhead of the listeners
pub struct ListenerManager<S: Send + Debug, I: Send + Debug + PartialEq + 'static> {
    model_listeners: Vec<(I, bool, EnvironmentListener)>,
    count_listeners: Vec<(I, CountListener)>,
    _s: ::std::marker::PhantomData<S>,
}

impl<S: Send + Debug, I: Send + Debug + PartialEq + 'static> Default for ListenerManager<S, I> {
    fn default() -> Self {
        ListenerManager {
            model_listeners: Vec::new(),
            count_listeners: Vec::new(),
            _s: ::std::marker::PhantomData,
        }
    }
}

impl<S: Send + Debug, I: Send + Debug + PartialEq + 'static> ListenerManager<S, I> {
    pub fn add_environment_listener(&mut self, identifier: I, listener: EnvironmentListener) {
        trace!("Adding model listener with id={:?}", identifier);
        let before = self.model_listeners.len();
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
        self.model_listeners.retain(|(id, _, _)| id.ne(&identifier));
        trace!(
            "Removed {} matches, still subscribed: {}",
            before - self.model_listeners.len(),
            self.model_listeners.len()
        );
        if before != self.model_listeners.len() {
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

    pub fn publish_environment_model(&mut self, model: Arc<RawMessage>) {
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

pub enum Command<U: Send + Debug, I: Send + Debug + Sized + 'static> {
    Update(Box<U>),
    Publish(RawMessage),
    SubscribeEnvironmentModel(I, EnvironmentListener),
    UnsubscribeEnvironmentModel(I),
    SubscribeListenerCount(I, CountListener),
    UnsubscribeListenerCount(I),
    ActivateEnvironmentModelSubscription(I),
    DeactivateEnvironmentModelSubscription(I),
}

impl<S: Send + Debug, I: Send + Debug + PartialEq + 'static> Command<S, I> {
    pub fn apply(self, algorithm: &mut Algorithm<S, I>) {
        trace!("Processing algorithm command");
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
        I: Send + Debug + PartialEq + 'static,
        F: FnMut(Command<S, I>),
    > Algorithm<S, I> for F
{
    fn update(&mut self, update: Box<S>) {
        self(Command::Update(update));
    }

    fn publish(&mut self, model: RawMessage) {
        self(Command::Publish(model));
    }

    fn subscribe_environment_model(&mut self, identifier: I, listener: EnvironmentListener) {
        self(Command::SubscribeEnvironmentModel(identifier, listener));
    }

    fn unsubscribe_environment_model(&mut self, identifier: I) {
        self(Command::UnsubscribeEnvironmentModel(identifier));
    }

    fn activate_environment_model_subscription(&mut self, identifier: I) {
        self(Command::ActivateEnvironmentModelSubscription(identifier));
    }

    fn deactivate_environment_model_subscription(&mut self, identifier: I) {
        self(Command::DeactivateEnvironmentModelSubscription(identifier));
    }

    fn subscribe_listener_count(&mut self, identifier: I, listener: CountListener) {
        self(Command::SubscribeListenerCount(identifier, listener));
    }

    fn unsubscribe_listener_count(&mut self, identifier: I) {
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
