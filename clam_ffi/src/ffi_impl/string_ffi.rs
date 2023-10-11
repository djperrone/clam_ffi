use std::{
    ffi::{c_char, CStr, CString},
    ptr::{null, null_mut},
};

use crate::utils::{error::FFIError, helpers};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct StringFFI {
    pub data: *mut u8,
    pub len: i32,
}

impl StringFFI {
    pub fn new(str: String) -> Self {
        StringFFI {
            // data: Self::alloc_to_c_char(data.clone()) as *mut u8,
            len: str.len() as i32,
            data: CString::new(str).unwrap().into_raw() as *mut u8,
            // str.into_raw()
        }
    }

    pub fn default() -> Self {
        StringFFI {
            // data: Self::alloc_to_c_char(data.clone()) as *mut u8,
            len: 0 as i32,
            data: null_mut(),
            // str.into_raw()
        }
    }

    pub fn as_string(&self) -> Result<String, FFIError> {
        // return Self::csharp_to_rust_utf8(self.data, self.len);
        unsafe {
            if self.data.is_null() {
                return Err(FFIError::NullPointerPassed);
            }
            let slice = std::slice::from_raw_parts(self.data, self.len as usize);
            match String::from_utf8(slice.to_vec()) {
                Ok(str) => Ok(str),
                Err(_) => Err(FFIError::InvalidStringPassed),
            }
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        return self.data;
    }

    // dont think this works as intended...
    // pub fn as_mut_ptr(&self) -> *mut u8 {
    //     return self.data;
    // }

    pub fn is_empty(&self) -> bool {
        return self.data.is_null();
    }

    pub fn free(&mut self) {
        unsafe {
            if !self.data.is_null() {
                {
                    drop(CString::from_raw(self.data as *mut i8));
                    self.len = 0;
                    self.data = null_mut();
                };
            }
        }
        // Self::free_string(self.data);
    }

    pub fn c_char_to_string(s: *const c_char) -> String {
        let c_str = unsafe {
            assert!(!s.is_null());

            CStr::from_ptr(s)
        };
        // debug!("cstr testing {:?}", c_str);
        let r_str = c_str.to_str().unwrap();

        String::from(r_str)
    }

    // pub fn to_string()

    // unsafe fn csharp_to_rust_utf8(utf8_str: *const u8, utf8_len: i32) -> Result<String, FFIError> {
    //     let slice = std::slice::from_raw_parts(utf8_str, utf8_len as usize);
    //     match String::from_utf8(slice.to_vec()) {
    //         // String::from_raw_parts
    //         Ok(str) => Ok(str),
    //         Err(_) => Err(FFIError::InvalidStringPassed),
    //     }
    // }

    // pub fn alloc_to_c_char(str: String) -> *mut c_char {
    //     let str = CString::new(str).unwrap();
    //     str.into_raw()
    // }

    // pub unsafe fn free_string(str: *mut u8) {
    //     if !str.is_null() {
    //         {
    //             CString::from_raw(str as *mut i8)
    //         };
    //     }
    // }
}
