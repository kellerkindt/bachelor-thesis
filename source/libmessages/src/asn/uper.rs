use asn::raw;

pub fn encode_to_new_buffer<T>(
    asn_type: &mut raw::asn_TYPE_descriptor_t,
    value: &T,
) -> Result<Vec<u8>, ()> {
    let mut vec = Vec::with_capacity(5120);

    let result = unsafe {
        unsafe extern "C" fn write_to_vec(
            buffer: *const ::std::os::raw::c_void,
            size: usize,
            application_specific_key: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int {
            #[allow(unknown_lints)]
            #[allow(transmute_ptr_to_ref)]
            let vec: &mut Vec<u8> = ::std::mem::transmute(application_specific_key as *mut Vec<u8>);
            let data = ::std::slice::from_raw_parts(buffer as *const u8, size);

            use std::io::Write;
            match vec.write_all(data) {
                Ok(_) => 0,
                Err(_) => -1,
            }
        }

        raw::uper_encode(
            asn_type as *mut raw::asn_TYPE_descriptor_t,
            value as *const T as *mut T as *mut ::std::os::raw::c_void,
            Some(write_to_vec),
            &mut vec as *mut _ as *mut ::std::os::raw::c_void,
        )
    };
    if result.encoded < 0 {
        Err(())
    } else {
        Ok(vec)
    }
}

pub unsafe fn encode<T>(
    asn_type: &mut raw::asn_TYPE_descriptor_t,
    value: &T,
    buffer: &mut [u8],
) -> Result<usize, ()> {
    let result = raw::uper_encode_to_buffer(
        asn_type as *mut raw::asn_TYPE_descriptor_t,
        value as *const T as *const ::std::os::raw::c_void as *mut ::std::os::raw::c_void,
        buffer.as_mut_ptr() as *mut ::std::os::raw::c_void,
        buffer.len(),
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

pub unsafe fn decode<T: Default>(
    asn_type: &mut raw::asn_TYPE_descriptor_t,
    buffer: &[u8],
) -> Result<Box<T>, ()> {
    let mut pointer: *mut T = Box::into_raw(Box::new(Default::default()));
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
        debug!("Freeing partially decoded data");
        drop(Box::from_raw(pointer));
        Err(())
    } else {
        trace!("fine");
        Ok(Box::from_raw(pointer))
    }
}
