/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug)]
pub struct std_shared_ptr {
    pub _address: u8,
}
pub type std_shared_ptr__Convertible = u8;
pub type EnvironmentFrame_t = [u64; 30usize];
pub type InitMessage_t = [u64; 30usize];
pub type SensorFrame_t = [u64; 27usize];
pub type RustEventListener = u64;
#[repr(C)]
#[derive(Debug)]
pub struct RustShimInternal {
    pub _bindgen_opaque_blob: [u64; 8usize],
}
#[test]
fn bindgen_test_layout_RustShimInternal() {
    assert_eq!(
        ::std::mem::size_of::<RustShimInternal>(),
        64usize,
        concat!("Size of: ", stringify!(RustShimInternal))
    );
    assert_eq!(
        ::std::mem::align_of::<RustShimInternal>(),
        8usize,
        concat!("Alignment of ", stringify!(RustShimInternal))
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct RustShim {
    pub publish_environment_frame: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut RustEventListener, arg2: *mut EnvironmentFrame_t),
    >,
    pub publish_init_message: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut RustEventListener, arg2: *mut InitMessage_t),
    >,
    pub internal: [u64; 2usize],
    pub eventListener: *mut RustEventListener,
}
#[test]
fn bindgen_test_layout_RustShim() {
    assert_eq!(
        ::std::mem::size_of::<RustShim>(),
        40usize,
        concat!("Size of: ", stringify!(RustShim))
    );
    assert_eq!(
        ::std::mem::align_of::<RustShim>(),
        8usize,
        concat!("Alignment of ", stringify!(RustShim))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<RustShim>())).publish_environment_frame as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RustShim),
            "::",
            stringify!(publish_environment_frame)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RustShim>())).publish_init_message as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RustShim),
            "::",
            stringify!(publish_init_message)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RustShim>())).internal as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RustShim),
            "::",
            stringify!(internal)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RustShim>())).eventListener as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RustShim),
            "::",
            stringify!(eventListener)
        )
    );
}
extern "C" {
    pub fn create_rust_shim_for_algorithm(
        config_file: *mut ::std::os::raw::c_char,
    ) -> *mut RustShim;
}
extern "C" {
    pub fn send_sensor_frame(shim: *mut RustShim, frame: *mut SensorFrame_t);
}
