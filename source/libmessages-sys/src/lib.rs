#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use]
extern crate log;

mod bindings;
pub use bindings::*;


unsafe impl Send for ClientRegistration {}
unsafe impl Sync for ClientRegistration {}

impl Default for ClientRegistration {
    fn default() -> Self {
        unsafe {
            ::std::mem::zeroed()
        }
    }
}

/*
impl Drop for ClientRegistration {
    fn drop(&mut self) {
        unsafe {
            free_content(&mut asn_DEF_ClientRegistration, self);
        }
    }
}*/



unsafe impl Send for UpdateSubscription {}
unsafe impl Sync for UpdateSubscription {}

impl Default for UpdateSubscription {
    fn default() -> Self {
        unsafe {
            ::std::mem::zeroed()
        }
    }
}

/*
impl Drop for UpdateSubscription {
    fn drop(&mut self) {
        unsafe {
            free_content(&mut asn_DEF_UpdateSubscription, self);
        }
    }
}*/



unsafe impl Send for SensorFrame {}
unsafe impl Sync for SensorFrame {}

impl Default for SensorFrame {
    fn default() -> Self {
        unsafe {
            let mut me : Self = ::std::mem::zeroed();
            asn_set_empty(&mut me.object_detections as *mut _ as *mut ::std::os::raw::c_void);
            me
        }
    }
}

/*
impl Drop for SensorFrame {
    fn drop(&mut self) {
        unsafe {
            free_content(&mut asn_DEF_SensorFrame, self);
        }
    }
}*/



unsafe impl Send for EnvironmentFrame {}
unsafe impl Sync for EnvironmentFrame {}

impl Default for EnvironmentFrame {
    fn default() -> Self {
        unsafe {
            let mut me : Self = ::std::mem::zeroed();
            asn_set_empty(&mut me.object_detections as *mut _ as *mut ::std::os::raw::c_void);
            asn_set_empty(&mut me.envelope.error_codes as *mut _ as *mut ::std::os::raw::c_void);
            me
        }
    }
}

/*
impl Drop for EnvironmentFrame {
    fn drop(&mut self) {
        unsafe {
            free_content(&mut asn_DEF_EnvironmentFrame, self);
        }
    }
}
*/



unsafe impl Send for RoadClearanceFrame {}
unsafe impl Sync for RoadClearanceFrame {}

impl Default for RoadClearanceFrame {
    fn default() -> Self {
        unsafe {
            let mut me : Self = ::std::mem::zeroed();
            asn_set_empty(&mut me.road_sections as *mut _ as *mut ::std::os::raw::c_void);
            me
        }
    }
}

/*
impl Drop for RoadClearanceFrame {
    fn drop(&mut self) {
        unsafe {
            free_content(&mut asn_DEF_RoadClearanceFrame, self);
        }
    }
}
*/


unsafe impl Send for SensorIdleFrame {}
unsafe impl Sync for SensorIdleFrame {}

impl Default for SensorIdleFrame {
    fn default() -> Self {
        unsafe {
            ::std::mem::zeroed()
        }
    }
}

/*
impl Drop for SensorIdleFrame {
    fn drop(&mut self) {
        unsafe {
            free_content(&mut asn_DEF_SensorIdleFrame, self);
        }
    }
}
*/


unsafe impl Send for UpdateStatus {}
unsafe impl Sync for UpdateStatus {}

impl Default for UpdateStatus {
    fn default() -> Self {
        unsafe {
            ::std::mem::zeroed()
        }
    }
}

/*
impl Drop for UpdateStatus {
    fn drop(&mut self) {
        unsafe {
            free_content(&mut asn_DEF_UpdateStatus, self);
        }
    }
}
*/


unsafe impl Send for InitMessage {}
unsafe impl Sync for InitMessage {}

impl Default for InitMessage {
    fn default() -> Self {
        unsafe {
            let mut me : Self = ::std::mem::zeroed();
            asn_set_empty(&mut me.sectors as *mut _ as *mut ::std::os::raw::c_void);
            asn_set_empty(&mut me.envelope.error_codes as *mut _ as *mut ::std::os::raw::c_void);
            me
        }
    }
}

/*
impl Drop for InitMessage {
    fn drop(&mut self) {
        unsafe {
            free_content(&mut asn_DEF_InitMessage, self);
        }
    }
}
*/


pub unsafe fn free_content<T>(asn_type: &mut asn_TYPE_descriptor_t, value: &T) {
    free(asn_type, value, true)
}

pub unsafe fn free<T>(asn_type: &mut asn_TYPE_descriptor_t, value: &T, only_content: bool) {
    trace!("asn_free type {:?}", asn_type);
    asn_type.free_struct.expect("free_struct is NULL, is the library not loaded?")(
        asn_type as *mut asn_TYPE_descriptor_t,
        value as *const T as *mut ::std::os::raw::c_void,
        if only_content {1} else {0} as ::std::os::raw::c_int,
    );
    trace!("freed");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn library_linked() {
        let _ = InitMessage::default();
    }
}