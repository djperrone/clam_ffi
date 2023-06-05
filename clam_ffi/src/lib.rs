mod ffi_impl;
mod utils;

use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use ffi_impl::handle::Handle;

#[no_mangle]
pub extern "C" fn get_answer() -> i32 {
    42
}

#[no_mangle]
pub unsafe extern "C" fn get_num_nodes(ptr: *mut Handle) -> i32 {
    match Handle::from_ptr(ptr).get_layout().as_ref() {
        Some(layout) => return layout.len() as i32,
        None => return 0,
    }
}

unsafe fn init_clam_helper<'a>(
    data_name: String,
    cardinality: u32,
) -> Result<Rc<RefCell<Handle<'a>>>, String> {
    // let data_name = match csharp_to_rust_utf8(data_name, name_len) {
    //     Ok(data_name) => data_name,
    //     Err(e) => {s
    //         debug!("{}", e);
    //         return Err(e);
    //     }
    // };
    // let mut handle = Box::from_raw(ptr as *mut Handle);
    let handle = Rc::new(RefCell::new(Handle::default()));

    let result = handle
        .as_ptr()
        .clone()
        .as_mut()
        .unwrap()
        .init_dataset(data_name.as_str());
    if result == 0 {
        debug!("failed to create dataset");
        return Err(format!("failed to create dataset {}", data_name));
    }

    let root = handle
        .as_ptr()
        .clone()
        .as_mut()
        .unwrap()
        .build_clam(cardinality as usize);
    match root {
        Ok(clam_root) => {
            handle
                .as_ptr()
                .clone()
                .as_mut()
                .unwrap()
                .set_root(clam_root.clone());

            handle
                .as_ptr()
                .clone()
                .as_mut()
                .unwrap()
                .create_reingold_layout();

            return Ok(handle);
        }
        Err(e) => {
            return Err(e);
        }
    }

    return Err("message".to_string());

    // Box::into_raw(handle);
    // return Ok(handle);
    // return handle;
}

#[no_mangle]
pub unsafe extern "C" fn init_clam(
    ptr: *mut *const Handle,
    data_name: *const u8,
    name_len: i32,
    cardinality: u32,
) -> i32 {
    let data_name = match csharp_to_rust_utf8(data_name, name_len) {
        Ok(data_name) => data_name,
        Err(e) => {
            debug!("{}", e);
            return 0;
        }
    };

    match init_clam_helper(data_name.clone(), cardinality) {
        Ok(handle) => {
            // let test = handle.as_ptr();
            *ptr = handle.as_ptr();
            // *ptr = handle.as_ref().borrow().to_ptr();
            std::mem::forget(handle);
            // let test = handle.to_owned().get_mut();
            // let test = handle.as_ptr()
            // .clone()
            // .as_ref()
            // .unwrap().to_ptr();
            // // Handle::to_ptr(test);
            // *ptr = handle.as_ref().borrow().to_ptr();
            debug!("built clam tree for {}", data_name);
            return 1;
        }
        Err(e) => {
            debug!("{}", e)
        }
    }

    return 0;
    // Box::into_raw(handle);
    // let mut handle = Box::from_raw(ptr as *mut Handle);
    // let result = handle.init_dataset(data_name.as_str());
    // if result == 0 {
    //     debug!("failed to create dataset");
    //     return 0;
    // }
    // handle.build_clam(cardinality as usize);

    // return 1;
}

#[no_mangle]
pub unsafe extern "C" fn create_reingold_layout(ptr: *mut Handle) -> i32 {
    // let mut handle = Box::from_raw(ptr as *mut Handle);
    Handle::from_ptr(ptr).create_reingold_layout();
    // handle.create_reingold_layout();

    if let Some(layout) = &Handle::from_ptr(ptr).get_layout() {
        return layout.len() as i32;
    }

    // Handle::to_ptr(self)

    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn free_reingold_layout(ptr: *mut Handle) -> () {
    let mut handle = Box::from_raw(ptr as *mut Handle);
    handle.free_reingold_layout();
}

pub unsafe fn csharp_to_rust_utf8(utf8_str: *const u8, utf8_len: i32) -> Result<String, String> {
    let slice = std::slice::from_raw_parts(utf8_str, utf8_len as usize);
    match String::from_utf8(slice.to_vec()) {
        Ok(str) => Ok(str),
        Err(_) => Err(String::from("invalid csharp_to_rust_utf8 conversion")),
    }
}
