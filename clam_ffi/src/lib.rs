mod ffi_impl;
mod utils;
use std::ffi::c_char;
use std::ffi::CStr;

use ffi_impl::handle::Handle;

// use ffi_impl::handle::Handle;
// use interoptopus::{ffi_function, ffi_type, function, Inventory, InventoryBuilder};

#[no_mangle]
pub extern "C" fn get_answer() -> i32 {
    42
}

#[no_mangle]
pub unsafe extern "C" fn init_clam(
    ptr: *mut *const Handle,
    data_name: *const u8,
    name_len : i32,
    cardinality: u32,
) -> u32 {
    let data_name = match csharp_to_rust_utf8(data_name, name_len){
        Ok(data_name) => data_name,
        Err(e) => {
            debug!("{}", e);
            return 0;
        }
    };

 
    debug!("creating tree {}", data_name);

    let mut handle = Handle::default();
    let t = Handle::init(&mut handle, data_name.as_str(), cardinality as usize);

    debug!("finished creating tree");

    // let len = match t.get_layout() {
    //     Some(pos_layout) => pos_layout.len(),
    //     None => 0,
    // };

    // unsafe {
    //     *ptr = t.to_ptr();
    // }

    return 0 as u32;
}

#[no_mangle]
pub unsafe extern "C" fn csharp_to_rust_utf8(utf8_str: *const u8, utf8_len: i32) -> Result<String, String>{
    let slice = std::slice::from_raw_parts(utf8_str, utf8_len as usize);
    match String::from_utf8(slice.to_vec()){
        Ok(str) => Ok(str),
        Err(e) => Err(String::from("invalid csharp_to_rust_utf8 conversion"))
    }
}
