use std::ffi::{c_char, CString};

pub fn alloc_to_c_char(str: String) -> *mut c_char {
    let str = CString::new(str).unwrap();
    str.into_raw()
}

pub fn free_c_char(str: *mut c_char) {
    unsafe { CString::from_raw(str) };
}
