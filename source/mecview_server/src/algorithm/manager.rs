use std::fmt::Debug;
use std::io::Error;
use std::net::SocketAddr;
use std::ops::IndexMut;
use std::sync::Arc;

use messages::RawMessage;

use super::Algorithm;
use super::CountListener;
use super::EnvironmentListener;

pub struct AlgorithmManager<S: Debug + Send, E: Debug + Send, T: FnMut(Box<S>) + Send + 'static> {
    model_listener: Vec<(SocketAddr, bool, EnvironmentListener<E>)>,
    count_listener: Vec<(SocketAddr, CountListener)>,
    sensor_frame_sink: T,
    _s: ::std::marker::PhantomData<S>,
}

impl<S: Debug + Send, E: Debug + Send, T: FnMut(Box<S>) + Send + 'static>
    AlgorithmManager<S, E, T>
{
    pub fn new(sensor_frame_sink: T) -> Self {
        AlgorithmManager {
            model_listener: Default::default(),
            count_listener: Default::default(),
            _s: Default::default(),
            sensor_frame_sink,
        }
    }

    fn push_model_listener(&mut self, id: SocketAddr, sink: EnvironmentListener<E>) {
        trace!("Adding model listener with id={}", id);
        let before = self.model_listener.len();
        self.model_listener.push((id, false, sink));

        trace!(
            "Now subscribed model listeners: {}",
            self.model_listener.len()
        );

        if before != self.model_listener.len() {
            self.on_model_count_changed(before);
        }
    }

    fn remove_model_listener(&mut self, remove: SocketAddr) {
        trace!("Removing model listener with id={}", remove);
        let before = self.model_listener.len();
        self.model_listener.retain(|(id, _, _)| id.ne(&remove));
        trace!(
            "Removed {} matches, still subscribed: {}",
            before - self.model_listener.len(),
            self.model_listener.len()
        );
        if before != self.model_listener.len() {
            self.on_model_count_changed(before);
        }
    }

    fn activate_model_listener(&mut self, id: SocketAddr) {
        self.model_listener
            .iter_mut()
            .filter(|(addr, _, _)| addr.eq(&id))
            .for_each(|(_, ref mut active, _)| *active = true);
    }

    fn deactivate_model_listener(&mut self, id: SocketAddr) {
        self.model_listener
            .iter_mut()
            .filter(|(addr, _, _)| addr.eq(&id))
            .for_each(|(_, ref mut active, _)| *active = false);
    }

    fn push_count_listener(&mut self, id: SocketAddr, mut sink: CountListener) {
        trace!("Adding count listener with id={}", id);
        if sink(0, self.model_listener.len()).is_ok() {
            self.count_listener.push((id, sink));
        }
        trace!(
            "Now subscribed count listeners: {}",
            self.count_listener.len()
        );
    }

    fn remove_count_listener(&mut self, remove: SocketAddr) {
        trace!("Removing count listener with id={}", remove);
        self.count_listener.retain(|(id, _)| id.ne(&remove));
        trace!(
            "Still subscribed count listeners. {}",
            self.count_listener.len()
        );
    }

    fn on_model_count_changed(&mut self, before: usize) {
        let after = self.model_listener.len();
        trace!("on_model_count_changed, before/after: {}/{}", before, after);
        retain_mut(&mut self.count_listener, |(_, listener)| {
            listener(before, after).is_ok()
        });
    }
}

impl<S: Debug + Send, E: Debug + Send, T: FnMut(Box<S>) + Send + 'static> Algorithm<S, E>
    for AlgorithmManager<S, E, T>
{
    type Identifier = SocketAddr;

    fn update(&mut self, update: Box<S>) -> Result<(), Error> {
        trace!("Update received: {:?}", update);
        (self.sensor_frame_sink)(update);
        Ok(())
    }

    fn publish(&mut self, model: RawMessage<E>) -> Result<(), Error> {
        let before = self.model_listener.len();
        let env = Arc::new(model);
        retain_mut(&mut self.model_listener, |(addr, active, listener)| {
            trace!("Sending model to ModelListener/{}", addr);
            !*active || listener(env.clone()).is_ok()
        });

        if before != self.model_listener.len() {
            self.on_model_count_changed(before);
        }
        Ok(())
    }

    fn subscribe_environment_model(
        &mut self,
        identifier: <Self as Algorithm<S, E>>::Identifier,
        sink: EnvironmentListener<E>,
    ) -> Result<(), Error> {
        self.push_model_listener(identifier, sink);
        Ok(())
    }

    fn unsubscribe_environment_model(
        &mut self,
        identifier: <Self as Algorithm<S, E>>::Identifier,
    ) -> Result<(), Error> {
        self.remove_model_listener(identifier);
        Ok(())
    }

    fn activate_environment_model_subscription(
        &mut self,
        identifier: <Self as Algorithm<S, E>>::Identifier,
    ) -> Result<(), Error> {
        self.activate_model_listener(identifier);
        Ok(())
    }

    fn deactivate_environment_model_subscription(
        &mut self,
        identifier: <Self as Algorithm<S, E>>::Identifier,
    ) -> Result<(), Error> {
        self.deactivate_model_listener(identifier);
        Ok(())
    }

    fn subscribe_listener_count(
        &mut self,
        identifier: <Self as Algorithm<S, E>>::Identifier,
        sink: CountListener,
    ) -> Result<(), Error> {
        self.push_count_listener(identifier, sink);
        Ok(())
    }

    fn unsubscribe_listener_count(
        &mut self,
        identifier: <Self as Algorithm<S, E>>::Identifier,
    ) -> Result<(), Error> {
        self.remove_count_listener(identifier);
        Ok(())
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
