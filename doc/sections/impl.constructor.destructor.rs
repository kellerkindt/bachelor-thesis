mod bindgen;

pub unsafe fn free_content<T>(asn_type: &mut asn_TYPE_descriptor_t, value: &T) {
    free_struct(asn_type, value, true)
}

pub unsafe fn free_struct<T>(asn_type: &mut asn_TYPE_descriptor_t, value: &T, only_content: bool) {
    trace!("asn_free type {:?}", asn_type);
    asn_type
        .free_struct
        .expect("free_struct is NULL, is the library not loaded?")(
        asn_type as *mut asn_TYPE_descriptor_t,
        value as *const T as *mut ::std::os::raw::c_void,
        if only_content { 1 } else { 0 } as ::std::os::raw::c_int,
    );
    trace!("freed");
}


unsafe impl Send for ClientRegistration {}
unsafe impl Sync for ClientRegistration {}

impl Default for ClientRegistration {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Drop for ClientRegistration {
    fn drop(&mut self) {
        unsafe {
            free_content(&mut asn_DEF_ClientRegistration, self);
        }
    }
}
