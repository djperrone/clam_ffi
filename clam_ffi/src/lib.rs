mod ffi_impl;
mod utils;

use std::{
    borrow::BorrowMut,
    cell::RefCell,
    ffi::{c_char, CString},
    rc::Rc,
};

use ffi_impl::{handle::Handle, node::NodeBaton};

#[no_mangle]
pub extern "C" fn get_answer() -> i32 {
    42
}

// Ou callback type. Note that we have to also add 'extern "C"' for it to work
type CBFn = extern "C" fn(f32, f32) -> f32;
type CBFnVoid = extern "C" fn(f32) -> ();
type CBFnVoidString = extern "C" fn(*mut c_char) -> ();
// type CBFnV = extern "C" fn(*mut c_char) -> ();
type CBFnInitNode = extern "C" fn(*mut c_char, *mut c_char, *mut c_char) -> ();
type CBFnNodeBaton = extern "C" fn(*mut NodeBaton, *mut c_char) -> ();

// Then we can just use this type as our function argument.
// Here we make a simple function that will call the callback with predefined argument.
#[no_mangle]
pub extern "C" fn retcall(callback: CBFn) -> f32 {
    return callback(2.0, 6.0);
}

#[no_mangle]
pub extern "C" fn voidcall(callback: CBFnVoid) -> () {
    callback(3.7);
}

#[no_mangle]
pub unsafe extern "C" fn init_node_objects(
    ptr: *mut Handle,
    set_csharp_node_id: CBFnInitNode,
) -> i32 {
    if !ptr.is_null() {
        return Handle::from_ptr(ptr).dfs_callback_set_node_names(set_csharp_node_id);
    }
    return 0;
    // let str = alloc_c_string();
    // callback(str);
    // free_c_string(str);
}

#[no_mangle]
pub unsafe extern "C" fn set_stringcb(callback: CBFnVoidString) -> () {
    let str = alloc_c_string();
    callback(str);
    free_c_string(str);
}

#[no_mangle]
pub unsafe extern "C" fn get_num_nodes(ptr: *mut Handle) -> i32 {
    Handle::from_ptr(ptr).get_num_nodes() + 1
}

#[no_mangle]
pub extern "C" fn alloc_c_string() -> *mut c_char {
    let str = CString::new("foo bar baz").unwrap();
    str.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn free_c_string(str: *mut c_char) {
    unsafe { CString::from_raw(str) };
}

unsafe fn init_clam_helper<'a>(
    data_name: String,
    cardinality: u32,
) -> Result<Rc<RefCell<Handle<'a>>>, String> {
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

            // handle
            //     .as_ptr()
            //     .clone()
            //     .as_mut()
            //     .unwrap()
            //     .create_reingold_layout();

            return Ok(handle);
        }
        Err(e) => {
            return Err(e);
        }
    }
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
            *ptr = handle.as_ptr();
            std::mem::forget(handle);

            debug!("built clam tree for {}", data_name);
            return 1;
        }
        Err(e) => {
            debug!("{}", e)
        }
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn create_reingold_layout(
    ptr: *mut Handle,
    send_baton: CBFnNodeBaton,
) -> i32 {
    if !ptr.is_null() {
        return Handle::from_ptr(ptr).create_reingold_layout(send_baton);

        // if let Some(layout) = &Handle::from_ptr(ptr).get_layout() {
        //     return layout.len() as i32;
        // }
    }

    return -2;
}

// #[no_mangle]
// pub unsafe extern "C" fn init_node(ptr: *mut Handle, node: *mut NodeBaton) -> () {
//     let handle = Handle::from_ptr(ptr);
// }

// #[no_mangle]
// pub unsafe extern "C" fn get_node_layout(ptr: *mut Handle) -> i32 {
//     if !ptr.is_null() {
//         Handle::from_ptr(ptr).create_reingold_layout();
//         if let Some(layout) = &Handle::from_ptr(ptr).get_layout() {
//             return layout.len() as i32;
//         }
//     }
//     return 0;
// }

#[no_mangle]
pub unsafe extern "C" fn free_reingold_layout(ptr: *mut Handle) -> () {
    // let mut handle = Box::from_raw(ptr as *mut Handle);
    // handle.free_reingold_layout();
    if !ptr.is_null() {
        Handle::from_ptr(ptr).free_reingold_layout();
    }
}

pub unsafe fn csharp_to_rust_utf8(utf8_str: *const u8, utf8_len: i32) -> Result<String, String> {
    let slice = std::slice::from_raw_parts(utf8_str, utf8_len as usize);
    match String::from_utf8(slice.to_vec()) {
        Ok(str) => Ok(str),
        Err(_) => Err(String::from("invalid csharp_to_rust_utf8 conversion")),
    }
}
