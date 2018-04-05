#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod bindings;
pub use bindings::*;

unsafe impl Send for ClientRegistration {}
unsafe impl Sync for ClientRegistration {}

unsafe impl Send for UpdateSubscription {}
unsafe impl Sync for UpdateSubscription {}

unsafe impl Send for SensorFrame {}
unsafe impl Sync for SensorFrame {}

unsafe impl Send for EnvironmentFrame {}
unsafe impl Sync for EnvironmentFrame {}

unsafe impl Send for RoadClearanceFrame {}
unsafe impl Sync for RoadClearanceFrame {}

unsafe impl Send for SensorIdleFrame {}
unsafe impl Sync for SensorIdleFrame {}

unsafe impl Send for UpdateStatus {}
unsafe impl Sync for UpdateStatus {}

unsafe impl Send for InitMessage {}
unsafe impl Sync for InitMessage {}