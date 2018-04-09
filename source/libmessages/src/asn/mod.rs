mod message;

pub extern crate libmessages_sys as raw;
pub use self::message::*;




pub fn uper_encode_to_new_buffer<T>(asn_type: &mut raw::asn_TYPE_descriptor_t, value: &T) -> Result<Vec<u8>, ()> {
    let mut pointer : *mut u8 = ::std::ptr::null_mut();
    let result = unsafe {
        raw::uper_encode_to_new_buffer(
            asn_type as *mut raw::asn_TYPE_descriptor_t,
            0 as *mut raw::asn_per_constraints_s,
            value as *const T as *mut T as *mut ::std::os::raw::c_void,
            (&mut pointer as *mut *mut u8) as *mut *mut ::std::os::raw::c_void,
        )
    };
    if result < 0 {
        Err(())
    } else {
        Ok(unsafe {
            Vec::from_raw_parts(pointer, result as usize, result as usize)
        })
    }
}

pub unsafe fn uper_encode<T>(asn_type: &mut raw::asn_TYPE_descriptor_t, value: &T, buffer: &mut [u8]) -> Result<usize, ()> {
    let result = raw::uper_encode_to_buffer(
        asn_type as *mut raw::asn_TYPE_descriptor_t,
        value as *const T as *const ::std::os::raw::c_void as *mut ::std::os::raw::c_void,
        buffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
        buffer.len()
    );

    trace!("result: {:?} for type: {:?}", result, asn_type);
    if result.encoded < 0 {
        warn!("Encoding failed: {:?}", result);
        Err(())
    } else {
        trace!("fine");
        // encoded is the length in BITS not bytes...
        Ok((result.encoded as usize + 7) / 8)
    }
}

pub unsafe fn uper_decode<T>(asn_type: &mut raw::asn_TYPE_descriptor_t, buffer: &[u8]) -> Result<Box<T>, ()> {
    let mut pointer : *mut T = ::std::ptr::null_mut();
    let result = raw::uper_decode_complete(
        ::std::ptr::null_mut(),
        asn_type as *mut raw::asn_TYPE_descriptor_t,
        (&mut pointer as *mut *mut T) as *mut *mut ::std::os::raw::c_void,
        buffer.as_ptr() as *mut ::std::os::raw::c_void,
        buffer.len(),
    );

    trace!("result: {:?} for type: {:?}", result, asn_type);
    if result.code != raw::asn_dec_rval_code_e_RC_OK {
        warn!("Decoding failed: {:?}", result);
        Err(())
    } else {
        trace!("fine");
        Ok(Box::from_raw(pointer))
    }
}


#[cfg(test)]
mod tests {
    use std::ptr;
    use std::mem;
    use std::os::raw;
    use raw::*;
    use super::super::tests::init_logger;


    #[test]
    fn library_link_valid_do_not_panic() {
        init_logger();
        unsafe {
            let mut buffer = [0u8; 32];
            let mut p = mem::zeroed::<PositionOffset>();
            p.position_north = 77 as raw::c_long;
            p.position_east = 77 as raw::c_long;
            p.std_dev_position_east = ptr::null_mut();
            p.std_dev_position_north = ptr::null_mut();

            trace!("{:?}", uper_encode_to_buffer(
                &mut asn_DEF_PositionOffset as *mut asn_TYPE_descriptor_s,
                &mut p as *mut _ as *mut raw::c_void,
                buffer.as_mut_ptr() as *mut raw::c_void,
                buffer.len() as usize
            ));
            let mut string = String::new();
            for byte in buffer.iter() {
                string.push_str(&format!("{:02x} ", byte));
            }
            trace!("{}", string)
        }
    }
}