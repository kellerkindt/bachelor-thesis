use std::io::Error;
use std::net::SocketAddr;
use std::ops::IndexMut;
use std::sync::Arc;

use messages::asn::raw::EnvironmentFrame;
use messages::asn::raw::SensorFrame;
use messages::asn::AsnMessage;
use messages::RawMessage;

use super::Algorithm;
use super::CountListener;
use super::EnvironmentListener;

#[derive(Default)]
pub struct SampleAlgorithm {
    model_listener: Vec<(SocketAddr, bool, EnvironmentListener<EnvironmentFrame>)>,
    count_listener: Vec<(SocketAddr, CountListener)>,
}

impl SampleAlgorithm {
    fn push_model_listener(&mut self, id: SocketAddr, sink: EnvironmentListener<EnvironmentFrame>) {
        trace!("Adding model listener with id={}", id);
        let len = self.model_listener.len();
        self.model_listener.push((id, false, sink));

        trace!(
            "Now subscribed model listeners: {}",
            self.model_listener.len()
        );

        if len != self.model_listener.len() {
            self.on_model_count_changed();
        }
    }

    fn remove_model_listener(&mut self, remove: SocketAddr) {
        trace!("Removing model listener with id={}", remove);
        let len = self.model_listener.len();
        self.model_listener.retain(|(id, _, _)| id.ne(&remove));
        trace!(
            "Removed {} matches, still subscribed: {}",
            len - self.model_listener.len(),
            self.model_listener.len()
        );
        if len != self.model_listener.len() {
            self.on_model_count_changed();
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
        if sink(self.model_listener.len()).is_ok() {
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

    fn on_update(&mut self, frame: &SensorFrame) {
        trace!("Sensor update received: {:?}", frame);
        let env = Self::environment_model(frame);
        let len = self.model_listener.len();

        Self::retain_mut(&mut self.model_listener, |(_, active, listener)| {
            !*active || listener(env.clone()).is_ok()
        });

        if len != self.model_listener.len() {
            self.on_model_count_changed();
        }
    }

    fn on_model_count_changed(&mut self) {
        let count = self.model_listener.len();
        trace!("on_model_count_changed, current count: {}", count);
        Self::retain_mut(&mut self.count_listener, |(_, listener)| {
            listener(count).is_ok()
        });
    }

    fn retain_mut<T, F: Fn(&mut T) -> bool>(vec: &mut Vec<T>, keep: F) {
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

    fn environment_model(frame: &SensorFrame) -> Arc<RawMessage<EnvironmentFrame>> {
        // TODO
        let mut env = EnvironmentFrame::default();
        env.header.timestamp = frame.header.timestamp;
        Arc::new(env.try_encode_uper().unwrap())
    }
}

impl Algorithm<SensorFrame, EnvironmentFrame> for SampleAlgorithm {
    type Identifier = SocketAddr;

    fn update(&mut self, update: Box<SensorFrame>) -> Result<(), Error> {
        self.on_update(&update);
        Ok(())
    }

    fn subscribe_environment_model(
        &mut self,
        identifier: <Self as Algorithm<SensorFrame, EnvironmentFrame>>::Identifier,
        sink: EnvironmentListener<EnvironmentFrame>,
    ) -> Result<(), Error> {
        self.push_model_listener(identifier, sink);
        Ok(())
    }

    fn unsubscribe_environment_model(
        &mut self,
        identifier: <Self as Algorithm<SensorFrame, EnvironmentFrame>>::Identifier,
    ) -> Result<(), Error> {
        self.remove_model_listener(identifier);
        Ok(())
    }

    fn activate_environment_model_subscription(
        &mut self,
        identifier: <Self as Algorithm<SensorFrame, EnvironmentFrame>>::Identifier,
    ) -> Result<(), Error> {
        self.activate_model_listener(identifier);
        Ok(())
    }

    fn deactivate_environment_model_subscription(
        &mut self,
        identifier: <Self as Algorithm<SensorFrame, EnvironmentFrame>>::Identifier,
    ) -> Result<(), Error> {
        self.deactivate_model_listener(identifier);
        Ok(())
    }

    fn subscribe_listener_count(
        &mut self,
        identifier: <Self as Algorithm<SensorFrame, EnvironmentFrame>>::Identifier,
        sink: CountListener,
    ) -> Result<(), Error> {
        self.push_count_listener(identifier, sink);
        Ok(())
    }

    fn unsubscribe_listener_count(
        &mut self,
        identifier: <Self as Algorithm<SensorFrame, EnvironmentFrame>>::Identifier,
    ) -> Result<(), Error> {
        self.remove_count_listener(identifier);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn retain_mut_removes_correct() {
        let mut list = vec![1, 2, 3, 4, 5];
        SampleAlgorithm::retain_mut(&mut list, |i| *i % 2 == 0);
        assert_eq!(2, list.len());
        assert!(list.contains(&2));
        assert!(list.contains(&4));
    }
}
