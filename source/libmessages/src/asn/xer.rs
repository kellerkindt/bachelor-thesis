use asn::raw;

pub fn encode_to_new_string<T>(
    asn_type: &mut raw::asn_TYPE_descriptor_t,
    value: &T,
) -> Result<String, ()> {
    let mut vec: Vec<u8> = Vec::new();
    let result = unsafe {
        unsafe extern "C" fn fill_vec(
            buffer: *const ::std::os::raw::c_void,
            size: usize,
            vec: *mut ::std::os::raw::c_void,
        ) -> i32 {
            let vec: &mut Vec<u8> = ::std::mem::transmute(vec);
            let slice: &[u8] = ::std::slice::from_raw_parts(buffer as *const u8, size);

            for byte in slice.iter() {
                vec.push(*byte);
            }

            0
        };

        raw::xer_encode(
            asn_type as *mut raw::asn_TYPE_descriptor_t,
            value as *const T as *mut T as *mut ::std::os::raw::c_void,
            raw::xer_encoder_flags_e_XER_F_BASIC,
            Some(fill_vec),
            &mut vec as *mut _ as *mut ::std::os::raw::c_void,
        )
    };
    if result.encoded < 0 {
        Err(())
    } else {
        String::from_utf8(vec).map_err(|_| ())
    }
}

pub fn decode<T>(asn_type: &mut raw::asn_TYPE_descriptor_t, xml: &str) -> Result<Box<T>, ()> {
    let mut pointer: *mut T = ::std::ptr::null_mut();
    let xml = xml.as_bytes();
    let result = unsafe {
        raw::xer_decode(
            ::std::ptr::null_mut(),
            asn_type as *mut raw::asn_TYPE_descriptor_t,
            (&mut pointer as *mut *mut T) as *mut *mut ::std::os::raw::c_void,
            xml.as_ptr() as *mut ::std::os::raw::c_void,
            xml.len(),
        )
    };
    trace!("result: {:?} for type: {:?}", result, asn_type);
    if result.code != raw::asn_dec_rval_code_e_RC_OK {
        if pointer != ::std::ptr::null_mut() {
            debug!("Freeing partially decoded data");
            unsafe {
                raw::free(asn_type, &*pointer as &T, false);
            }
        }
        Err(())
    } else {
        Ok(unsafe { Box::from_raw(pointer) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_sample_to_new_buffer() {
        super::super::tests::init_logger();
        let reg = raw::ClientRegistration::default();
        let str =
            encode_to_new_string(unsafe { &mut raw::asn_DEF_ClientRegistration }, &reg).unwrap();
        assert_eq!(
            "<ClientRegistration>
    <type><sensor/></type>
</ClientRegistration>
",
            &str
        );
    }

    #[test]
    fn decode_sample() {
        super::super::tests::init_logger();
        let decoded: Box<raw::ClientRegistration> = decode(
            unsafe { &mut raw::asn_DEF_ClientRegistration },
            "<ClientRegistration><type><sensor/></type></ClientRegistration>",
        ).unwrap();
        assert_eq!(
            raw::ClientType_ClientType_sensor as raw::ClientType_t,
            decoded.type_
        );
        assert_eq!(::std::ptr::null_mut(), decoded.covered_area);
        assert_eq!(::std::ptr::null_mut(), decoded.minimum_message_period);
    }
}
