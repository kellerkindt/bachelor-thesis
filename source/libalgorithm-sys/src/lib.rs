#![allow(dead_code)]

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod bindings;

pub struct ExternalAlgorithm<
    I: Send,
    E: Send,
    S: Send,
    FE: FnMut(&E) + Send + 'static,
    FI: FnMut(&I) + Send + 'static,
> {
    shim: *mut bindings::RustShim,
    environment_frame_publisher: FE,
    init_message_publisher: FI,
    _e: ::std::marker::PhantomData<E>,
    _s: ::std::marker::PhantomData<S>,
    _i: ::std::marker::PhantomData<I>,
}

unsafe impl<I: Send, E: Send, S: Send, FE: FnMut(&E) + Send + 'static, FI: FnMut(&I) + Send + 'static>
    Send for ExternalAlgorithm<I, E, S, FE, FI>
{
}

impl<I: Send, E: Send, S: Send, FE: FnMut(&E) + Send + 'static, FI: FnMut(&I) + Send + 'static>
    ExternalAlgorithm<I, E, S, FE, FI>
{
    pub unsafe fn new(
        config_file: &str,
        environment_frame_publisher: FE,
        init_message_publisher: FI,
    ) -> Result<Box<ExternalAlgorithm<I, E, S, FE, FI>>, ()> {
        let mut algorithm = Box::new(ExternalAlgorithm {
            shim: {
                let cstring = ::std::ffi::CString::new(config_file).unwrap();
                let cstring = cstring.into_raw();

                let shim = bindings::shim_create(cstring);
                let _ = ::std::ffi::CString::from_raw(cstring); // drop it gracefully

                if shim.is_null() {
                    return Err(());
                } else {
                    shim
                }
            },
            environment_frame_publisher,
            init_message_publisher,
            _e: Default::default(),
            _s: Default::default(),
            _i: Default::default(),
        });
        (*algorithm.shim).instance = algorithm.as_mut() as *mut _ as *mut bindings::RustInstance;
        (*algorithm.shim).publish_environment_frame = Some(Self::publish_environment_frame);
        (*algorithm.shim).publish_init_message = Some(Self::publish_init_message);
        (*algorithm.shim).drop_sensor_frame = Some(Self::drop_sensor_frame);
        Ok(algorithm)
    }

    #[no_mangle]
    unsafe extern "C" fn publish_environment_frame(
        instance: *mut bindings::RustInstance,
        frame: *mut bindings::EnvironmentFrame_t,
    ) {
        let myself: &mut ExternalAlgorithm<I, E, S, FE, FI> = ::std::mem::transmute(instance);
        let frame: &E = ::std::mem::transmute(frame as *mut E);
        (myself.environment_frame_publisher)(frame);
    }

    #[no_mangle]
    unsafe extern "C" fn publish_init_message(
        instance: *mut bindings::RustInstance,
        frame: *mut bindings::InitMessage_t,
    ) {
        let myself: &mut ExternalAlgorithm<I, E, S, FE, FI> = ::std::mem::transmute(instance);
        let frame: &I = ::std::mem::transmute(frame as *mut I);
        (myself.init_message_publisher)(frame);
    }

    #[no_mangle]
    unsafe extern "C" fn drop_sensor_frame(frame: *mut bindings::SensorFrame_t) {
        // turning the raw pointer in the box causes
        // it to be dropped correctly by the rust deallocator
        Box::from_raw(frame as *mut S);
    }

    pub fn send_sensor_frame(&mut self, frame: Box<S>) {
        unsafe {
            bindings::shim_send_sensor_frame(
                self.shim,
                Box::into_raw(frame) as *mut S as *mut bindings::SensorFrame_t,
            );
        }
    }
}

impl<I: Send, E: Send, S: Send, FE: FnMut(&E) + Send + 'static, FI: FnMut(&I) + Send + 'static> Drop
    for ExternalAlgorithm<I, E, S, FE, FI>
{
    fn drop(&mut self) {
        if !self.shim.is_null() {
            unsafe {
                bindings::shim_destroy(self.shim);
            }
            self.shim = ::std::ptr::null_mut();
        }
    }
}
