pub extern crate libmessages_sys as raw;

#[derive(Debug)]
pub enum Message {
    Registration(Box<raw::ClientRegistration>),
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe {
            match self {
                Message::Registration(ref mut registration) => asn_free(&mut raw::asn_DEF_ClientRegistration, ::std::mem::replace(registration, ::std::mem::uninitialized())),
            };
        }
    }
}

unsafe fn asn_free<T>(asn_type: &mut raw::asn_TYPE_descriptor_t, value: Box<T>) {
    asn_type.free_struct.expect("free_struct is NULL, is the library not loaded?")(
        asn_type as *mut raw::asn_TYPE_descriptor_t,
        Box::into_raw(value) as *mut ::std::os::raw::c_void,
        0 as ::std::os::raw::c_int
    );
}


#[cfg(test)]
mod tests {
    use std::ptr;
    use std::mem;
    use std::os::raw;
    use raw::*;

    #[test]
    fn do_not_panic() {
        unsafe {
            let mut buffer = [0u8; 16];
            let mut p = mem::zeroed::<PositionOffset>();
            p.position_north = 77 as raw::c_long;
            p.position_east = 77 as raw::c_long;
            p.std_dev_position_east = ptr::null_mut();
            p.std_dev_position_north = ptr::null_mut();

            println!("{:?}", uper_encode_to_buffer(
                &mut asn_DEF_PositionOffset as *mut asn_TYPE_descriptor_s,
                &mut p as *mut _ as *mut raw::c_void,
                buffer.as_mut_ptr() as *mut raw::c_void,
                buffer.len() as usize
            ));
            println!();
            for byte in buffer.iter() {
                print!("{:02x} ", byte);
            }
            println!();
        }
    }
}
