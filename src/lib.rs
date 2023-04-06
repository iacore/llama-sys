#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::{ffi::c_void, mem, ptr::null};

    #[test]
    fn test_ggml_init() {
        unsafe {
            let ctx = ggml_init(ggml_init_params {
                mem_size: 16 * 1024 * 1024,
                mem_buffer: null::<c_void>() as *mut c_void,
                no_alloc: false,
            });
            ggml_free(ctx);
        }
    }

    #[test]
    fn test_llama_token_bos() {
        unsafe {
            let tok = llama_token_bos();
            assert_eq!(tok, 1);
        }
    }
}
