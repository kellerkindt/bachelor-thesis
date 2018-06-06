#![allow(dead_code)]

#[macro_use]
extern crate log;

extern crate libalgorithm;
extern crate libasn;
extern crate libmessages;
extern crate libshim_sys as bindings;

use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;

use libalgorithm::Algorithm;
use libalgorithm::CountListener;
use libalgorithm::EnvironmentListener;
use libalgorithm::ListenerManager;
use libasn::raw::EnvironmentFrame;
use libasn::raw::InitMessage;
use libasn::raw::SensorFrame;
use libasn::AsnMessage;
use libmessages::RawMessage;
use bindings::AlgorithmShim;
use bindings::Publisher;

pub struct ExternalAlgorithm<I: Send + Debug + PartialEq + 'static> {
    mgr: Arc<Mutex<ListenerManager<SensorFrame, I>>>,
    shim: Box<AlgorithmShim<InitMessage, EnvironmentFrame, SensorFrame, EnvironmentFramePublisher<I>, InitFramePublisher<I>>>,
}

impl<I: Send + Debug + PartialEq + 'static> ExternalAlgorithm<I> {
    pub fn new(config_file: &str) -> Result<Self, ()> {
        let mgr: Arc<Mutex<ListenerManager<SensorFrame, I>>> =
            Arc::new(Mutex::new(ListenerManager::default()));
        let shim: Box<
            AlgorithmShim<InitMessage, EnvironmentFrame, SensorFrame, _, _>,
        > = unsafe {
            bindings::AlgorithmShim::new(
                config_file,
                EnvironmentFramePublisher(mgr.clone()),
                InitFramePublisher(mgr.clone()),
            )?
        };
        Ok(ExternalAlgorithm { mgr, shim })
    }
}


impl<I: Send + Debug + PartialEq + 'static> Algorithm<SensorFrame, I>
    for ExternalAlgorithm<I>
{
    fn update(&mut self, update: Box<SensorFrame>) {
        self.shim.send_sensor_frame(update);
    }

    fn publish(&mut self, model: RawMessage) {
        match self.mgr.lock() {
            Err(e) => error!("Failed to lock ListenerManager: {:?}", e),
            Ok(ref mut mgr) => mgr.publish_environment_model(Arc::new(model)),
        };
    }

    fn subscribe_environment_model(
        &mut self,
        identifier: I,
        listener: EnvironmentListener,
    ) {
        match self.mgr.lock() {
            Err(e) => error!("Failed to lock ListenerManager: {:?}", e),
            Ok(ref mut mgr) => mgr.add_environment_listener(identifier, listener),
        };
    }

    fn unsubscribe_environment_model(&mut self, identifier: I) {
        match self.mgr.lock() {
            Err(e) => error!("Failed to lock ListenerManager: {:?}", e),
            Ok(ref mut mgr) => mgr.remove_environment_listener(identifier),
        };
    }

    fn activate_environment_model_subscription(&mut self, identifier: I) {
        match self.mgr.lock() {
            Err(e) => error!("Failed to lock ListenerManager: {:?}", e),
            Ok(ref mut mgr) => mgr.activate_environment_model_listener(identifier),
        };
    }

    fn deactivate_environment_model_subscription(&mut self, identifier: I) {
        match self.mgr.lock() {
            Err(e) => error!("Failed to lock ListenerManager: {:?}", e),
            Ok(ref mut mgr) => mgr.deactivate_environment_model_listener(identifier),
        };
    }

    fn subscribe_listener_count(&mut self, identifier: I, listener: CountListener) {
        match self.mgr.lock() {
            Err(e) => error!("Failed to lock ListenerManager: {:?}", e),
            Ok(ref mut mgr) => mgr.add_count_listener(identifier, listener),
        };
    }

    fn unsubscribe_listener_count(&mut self, identifier: I) {
        match self.mgr.lock() {
            Err(e) => error!("Failed to lock ListenerManager: {:?}", e),
            Ok(ref mut mgr) => mgr.remove_count_listener(identifier),
        };
    }
}



struct EnvironmentFramePublisher<I: Send + Debug + PartialEq + 'static>(Arc<Mutex<ListenerManager<SensorFrame, I>>>);

impl<I: Send + Debug + PartialEq + 'static> Publisher<EnvironmentFrame> for EnvironmentFramePublisher<I> {
    fn publish(&mut self, frame: &EnvironmentFrame) {
        match frame.try_encode_uper() {
            Err(e) => error!("Failed to encode EnvironmentFrame to uper RawMessage {:?}", e),
            Ok(msg) => match self.0.lock() {
                Err(e) => error!("Failed to lock ListenerManager for publishing EnvironmentFrame: {:?}", e),
                Ok(mut mgr) => mgr.publish_environment_model(Arc::new(msg)),
            },
        };
    }
}

struct InitFramePublisher<I: Send + Debug + PartialEq + 'static>(Arc<Mutex<ListenerManager<SensorFrame, I>>>);

impl<I: Send + Debug + PartialEq + 'static> Publisher<InitMessage> for InitFramePublisher<I> {
    fn publish(&mut self, _: &InitMessage) {
        warn!("Received InitMessage from external algorithm, but ignoring it");
    }
}
