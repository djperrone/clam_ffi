use std::ffi::{c_char, CString};

pub fn alloc_to_c_char(str: String) -> *mut c_char {
    let str = CString::new(str).unwrap();
    str.into_raw()
}

pub fn free_c_char(str: *mut c_char) {
    unsafe { CString::from_raw(str) };
}

pub unsafe fn csharp_to_rust_utf8(utf8_str: *const u8, utf8_len: i32) -> Result<String, String> {
    let slice = std::slice::from_raw_parts(utf8_str, utf8_len as usize);
    match String::from_utf8(slice.to_vec()) {
        Ok(str) => Ok(str),
        Err(_) => Err(String::from("invalid csharp_to_rust_utf8 conversion")),
    }
}
