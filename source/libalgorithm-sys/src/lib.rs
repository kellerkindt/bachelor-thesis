#![allow(dead_code)]

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod bindings;

pub struct ExternalAlgorithm<I: Send, E: Send, S: Send> {
    shim: *mut bindings::RustShim,
    environment_frame_publisher: Box<FnMut(Box<E>) + Send + 'static>,
    init_message_publisher: Box<FnMut(Box<I>) + Send + 'static>,
    _e: ::std::marker::PhantomData<E>,
    _s: ::std::marker::PhantomData<S>,
    _i: ::std::marker::PhantomData<I>,
}

unsafe impl<I: Send, E: Send, S: Send> Send for ExternalAlgorithm<I, E, S> {}

impl<I: Send, E: Send, S: Send> ExternalAlgorithm<I, E, S> {
    pub fn new(
        config_file: &str,
        environment_frame_publisher: Box<FnMut(Box<E>) + Send + 'static>,
        init_message_publisher: Box<FnMut(Box<I>) + Send + 'static>,
    ) -> Box<ExternalAlgorithm<I, E, S>> {
        unsafe {
            let mut algorithm = Box::new(ExternalAlgorithm {
                shim: {
                    let config_file : String = config_file.into();
                    let cstring = ::std::ffi::CString::new(&config_file as &str).unwrap();
                    println!("Calling create_rust_shim_for_algorithm");
                    bindings::create_rust_shim_for_algorithm(
                        cstring.into_raw()
                    )
                },
                environment_frame_publisher,
                init_message_publisher,
                _e: Default::default(),
                _s: Default::default(),
                _i: Default::default(),
            });
            (*algorithm.shim).eventListener =
                algorithm.as_mut() as *mut _ as *mut bindings::RustEventListener;
            (*algorithm.shim).publish_environment_frame = Some(Self::publish_environment_frame);
            (*algorithm.shim).publish_init_message = Some(Self::publish_init_message);
            algorithm
        }
    }

    #[no_mangle]
    unsafe extern "C" fn publish_environment_frame(
        instance: *mut bindings::RustEventListener,
        frame: *mut bindings::EnvironmentFrame_t,
    ) {
        let myself: &mut ExternalAlgorithm<I, E, S> = ::std::mem::transmute(instance);
        (myself.environment_frame_publisher)(Box::from_raw(frame as *mut E));
    }

    #[no_mangle]
    unsafe extern "C" fn publish_init_message(
        instance: *mut bindings::RustEventListener,
        frame: *mut bindings::InitMessage_t,
    ) {
        let myself: &mut ExternalAlgorithm<I, E, S> = ::std::mem::transmute(instance);
        (myself.init_message_publisher)(Box::from_raw(frame as *mut I));
    }

    pub fn send_sensor_frame(&mut self, frame: Box<S>) {
        unsafe {
            bindings::send_sensor_frame(
                self.shim,
                Box::into_raw(frame) as *mut S as *mut bindings::SensorFrame_t,
            );
        }
    }
}
