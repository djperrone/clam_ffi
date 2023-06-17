use std::ffi::{c_char, CString};

pub fn alloc_to_c_char(str: String) -> *mut c_char {
    let str = CString::new(str).unwrap();
    str.into_raw()
}

pub fn free_c_char(str: *mut c_char) {
    if !str.is_null() {
        unsafe { CString::from_raw(str) };
    } else {
        debug!("tried to free null string");
    }
}

pub fn free_string(str: *mut u8) {
    if !str.is_null() {
        unsafe { CString::from_raw(str as *mut i8) };
    } else {
        debug!("tried to free null string");
    }
}

pub unsafe fn csharp_to_rust_utf8(utf8_str: *const u8, utf8_len: i32) -> Result<String, String> {
    let slice = std::slice::from_raw_parts(utf8_str, utf8_len as usize);
    match String::from_utf8(slice.to_vec()) {
        Ok(str) => Ok(str),
        Err(_) => Err(String::from("invalid csharp_to_rust_utf8 conversion")),
    }
}

pub fn hex_to_binary(hex_string: String) -> String {
    let mut binary_string = String::new();

    for hex_char in hex_string.chars() {
        match hex_char {
            '0' => binary_string.push_str("0000"),
            '1' => binary_string.push_str("0001"),
            '2' => binary_string.push_str("0010"),
            '3' => binary_string.push_str("0011"),
            '4' => binary_string.push_str("0100"),
            '5' => binary_string.push_str("0101"),
            '6' => binary_string.push_str("0110"),
            '7' => binary_string.push_str("0111"),
            '8' => binary_string.push_str("1000"),
            '9' => binary_string.push_str("1001"),
            'A' | 'a' => binary_string.push_str("1010"),
            'B' | 'b' => binary_string.push_str("1011"),
            'C' | 'c' => binary_string.push_str("1100"),
            'D' | 'd' => binary_string.push_str("1101"),
            'E' | 'e' => binary_string.push_str("1110"),
            'F' | 'f' => binary_string.push_str("1111"),
            _ => {
                // Handle invalid characters here if desired
                // For simplicity, we'll skip invalid characters
            }
        }
    }

    binary_string
}
