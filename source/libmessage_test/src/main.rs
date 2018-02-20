
extern crate libc;

mod bindings;

use libc::c_void;

use std::ptr;
use std::mem;
use std::os::raw;
use std::ffi::CString;

use bindings::asn_enc_rval_t;
use bindings::asn_dec_rval_t;
use bindings::asn_TYPE_descriptor_s;
use bindings::asn_codec_ctx_s;
use bindings::asn_struct_ctx_t;

#[repr(C)]
pub struct PositionOffset {
    pub position_north: raw::c_long,
    pub position_east: raw::c_long,
    pub std_dev_position_north: *mut raw::c_long,
    pub std_dev_position_east: *mut raw::c_long,

    pub _asn_ctx: asn_struct_ctx_t,
}



#[link(name = "messages", kind = "static")]
extern {
    static asn_DEF_PositionOffset : asn_TYPE_descriptor_s;

    fn uper_decode_complete(
            opt_codec_ctx: *const asn_codec_ctx_s,
            type_descriptor: *const asn_TYPE_descriptor_s,
            struct_ptr: *const *const raw::c_void,
            buffer: *const raw::c_void,
            size: usize
    ) -> asn_dec_rval_t;

    fn uper_encode_to_buffer(
        type_descriptor: *const asn_TYPE_descriptor_s,
        struct_ptr: *const raw::c_void,
        buffer: *mut raw::c_void,
        buffer_size: usize,
    ) -> asn_enc_rval_t;

    fn create_position_offset() -> *mut PositionOffset;
}

#[link(name = "blubber", kind = "static")]
extern {
    fn blubber(a: libc::c_int) -> libc::c_int;
}


fn main() {
    let input = 4;
    let output = 2; //unsafe { double_input(input) };
    unsafe {
        // let _ = per_data_string(0 as *mut asn_per_outp_t);
        println!("{}", blubber(2));
    }
    unsafe {
        let mut buffer = [0u8; 16];
        let mut p = mem::zeroed::<PositionOffset>();
        p.position_north = 77 as libc::c_long;
        p.position_east = 77 as libc::c_long;
        p.std_dev_position_east = ptr::null_mut();
        p.std_dev_position_north = ptr::null_mut();

        println!("{:?}", uper_encode_to_buffer(
            &asn_DEF_PositionOffset as *const asn_TYPE_descriptor_s,
            &p as *const _ as *const raw::c_void,
            buffer.as_mut_ptr() as *mut raw::c_void,
            buffer.len() as usize
        ));
        println!();
        for byte in buffer.iter() {
            print!("{:02x} ", byte);
        }
        println!();
        // println!("encoded length: {}", *(&count as *const _ as *const libc::c_int));
        /*
        uper_decode_complete(
            0 as *const asn_codec_ctx_s,
            0 as *const asn_TYPE_descriptor_s,
            0 as *const *const libc::c_void,
            0 as *const libc::c_void,
            0 as libc::size_t
        );*/
    }
    println!("{} * 2 = {}", input, output);
}
