
use std::io::Error;
use std::sync::Arc;
use std::ops::IndexMut;
use std::net::SocketAddr;

use messages::RawMessage;
use messages::asn::AsnMessage;
use messages::asn::raw::SensorFrame;
use messages::asn::raw::EnvironmentFrame;

use super::Algorithm;

#[derive(Default)]
pub struct SampleAlgorithm {
    model_listener: Vec<(SocketAddr, Box<FnMut(Arc<RawMessage<EnvironmentFrame>>) -> Result<(), Error>+Send>)>,
    count_listener: Vec<(SocketAddr, Box<FnMut(usize) -> Result<(), Error>+Send>)>,
}

impl SampleAlgorithm {

    fn push_model_listener(&mut self, id: SocketAddr, sink: Box<FnMut(Arc<RawMessage<EnvironmentFrame>>) -> Result<(), Error>+Send>) {
        trace!("Adding model listener with id={}", id);
        let len = self.model_listener.len();
        self.model_listener.push((id, sink));
        if len != self.model_listener.len() {
            self.on_model_count_changed();
        }
    }

    fn remove_model_listener(&mut self, remove: SocketAddr) {
        trace!("Removing model listener with id={}", remove);
        let len = self.model_listener.len();
        self.model_listener.retain(|(id, _)| id.ne(&remove));
        trace!("Removed {} matches", len - self.model_listener.len());
        if len != self.model_listener.len() {
            self.on_model_count_changed();
        }
    }

    fn push_count_listener(&mut self, id: SocketAddr, mut sink: Box<FnMut(usize) -> Result<(), Error>+Send>) {
        trace!("Adding count listener with id={}", id);
        if sink(self.model_listener.len()).is_ok() {
            self.count_listener.push((id, sink));
        }
    }


    fn remove_count_listener(&mut self, remove: SocketAddr) {
        trace!("Removing count listener with id={}", remove);
        self.count_listener.retain(|(id, _)| id.ne(&remove));
    }

    fn on_update(&mut self, frame: &SensorFrame) {
        trace!("Sensor update received: {:?}", frame);
        let env = Self::environment_model(frame);
        let len = self.model_listener.len();

        Self::retain_mut(&mut self.model_listener, |(_, listener)| {
            listener(env.clone()).is_ok()
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
        for mut i in 0..vec.len() {
            if !keep(vec.index_mut(i)) {
                vec.swap_remove(i);
                i -= 1;
            }
        }
    }

    fn environment_model(frame: &SensorFrame) -> Arc<RawMessage<EnvironmentFrame>> {
        // TODO
        let mut env = EnvironmentFrame::default();
        env.header.timestamp = frame.header.timestamp;
        Arc::new(env.encode().unwrap())
    }
}

impl Algorithm<SensorFrame, EnvironmentFrame> for SampleAlgorithm {
    type Identifier = SocketAddr;

    fn update(&mut self, update: Box<SensorFrame>) -> Result<(), Error> {
        self.on_update(&update);
        Ok(())
    }

    fn subscribe_environment_model(&mut self, identifier: <Self as Algorithm<SensorFrame, EnvironmentFrame>>::Identifier, sink: Box<FnMut(Arc<RawMessage<EnvironmentFrame>>) -> Result<(), Error>+Send+'static>) -> Result<(), Error> {
        self.push_model_listener(identifier, sink);
        Ok(())
    }

    fn unsubscribe_environment_model(&mut self, identifier: <Self as Algorithm<SensorFrame, EnvironmentFrame>>::Identifier) -> Result<(), Error> {
        self.remove_model_listener(identifier);
        Ok(())
    }

    fn subscribe_listener_count(&mut self, identifier: <Self as Algorithm<SensorFrame, EnvironmentFrame>>::Identifier, sink: Box<FnMut(usize) -> Result<(), Error>+Send+'static>) -> Result<(), Error> {
        self.push_count_listener(identifier, sink);
        Ok(())
    }

    fn unsubscribe_listener_count(&mut self, identifier: <Self as Algorithm<SensorFrame, EnvironmentFrame>>::Identifier) -> Result<(), Error> {
        self.remove_count_listener(identifier);
        Ok(())
    }
}