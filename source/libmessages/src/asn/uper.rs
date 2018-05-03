use asn::raw;

pub fn encode_to_new_buffer<T>(asn_type: &mut raw::asn_TYPE_descriptor_t, value: &T) -> Result<Vec<u8>, ()> {
    let mut pointer : *mut u8 = ::std::ptr::null_mut();
    let result = unsafe {
        raw::uper_encode_to_new_buffer(
            asn_type as *mut raw::asn_TYPE_descriptor_t,
            ::std::ptr::null_mut(),
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

pub unsafe fn encode<T>(asn_type: &mut raw::asn_TYPE_descriptor_t, value: &T, buffer: &mut [u8]) -> Result<usize, ()> {
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

pub unsafe fn decode<T>(asn_type: &mut raw::asn_TYPE_descriptor_t, buffer: &[u8]) -> Result<Box<T>, ()> {
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
        if pointer != ::std::ptr::null_mut() {
            debug!("Freeing partially decoded data");
            raw::free(asn_type, &*pointer as &T, false);
        }
        Err(())
    } else {
        trace!("fine");
        Ok(Box::from_raw(pointer))
    }
}
