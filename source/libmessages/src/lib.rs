pub extern crate libmessages_sys as raw;

#[derive(Debug)]
pub enum Message {
    Registration(raw::ClientRegistration),
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
