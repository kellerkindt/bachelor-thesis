use std::os::raw::c_void;
		
#[link(name = "messages", kind = "static")]
extern {
    type asn_TYPE_descriptor_s;
    type asn_enc_rval_t;
	
    fn uper_encode_to_buffer(
        type_descriptor: *const asn_TYPE_descriptor_s,
        struct_ptr: *const c_void,
        buffer: *mut c_void,
        buffer_size: usize,
    ) -> asn_enc_rval_t;
}
