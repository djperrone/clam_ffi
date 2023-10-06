use crate::utils::{error::FFIError, helpers};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct StringFFI {
    pub data: *mut u8,
    pub len: i32,
}

impl StringFFI {
    pub fn new(data: String) -> Self {
        StringFFI {
            data: helpers::alloc_to_c_char(data.clone()) as *mut u8,
            len: data.len() as i32,
        }
    }

    pub unsafe fn as_string(&self) -> Result<String, FFIError> {
        return helpers::csharp_to_rust_utf8(self.data, self.len);
    }

    pub fn as_ptr(&self) -> *const u8 {
        return self.data;
    }

    // dont think this works as intended...
    pub fn as_mut_ptr(&self) -> *mut u8 {
        return self.data;
    }

    pub fn is_empty(&self) -> bool {
        return self.data.is_null();
    }

    pub fn free(&mut self) {
        helpers::free_string(self.data);
        self.len = 0;
    }
}
