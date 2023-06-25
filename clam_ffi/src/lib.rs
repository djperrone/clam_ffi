//need to use partial struct to pass by reference and access sub structs
// pass by pointer with classes but cant seem to access sub structs

use std::{
    cell::RefCell,
    ffi::{c_char, CStr},
    rc::Rc,
};
mod core;
mod ffi_impl;
mod tests;
mod utils;
use ffi_impl::{
    handle::Handle,
    node::{NodeData, StringFFI},
};
use utils::{error::FFIError, helpers};
type CBFnNodeVistor = extern "C" fn(Option<&NodeData>) -> ();

// type OutHandlePtr1 = Option<& mut *mut Handle>;
type OutHandlePtr<'a> = Option<&'a mut *mut Handle>;

// type InHandlePtr = Option<& mut Handle>;
type InHandlePtr<'a> = Option<&'a mut Handle>;

#[no_mangle]
pub unsafe extern "C" fn get_cluster_data(
    context: InHandlePtr,
    incoming: Option<&NodeData>,
    outgoing: Option<&mut NodeData>,
) -> FFIError {
    if let Some(handle) = context {
        if let Some(in_node) = incoming {
            if let Some(out_node) = outgoing {
                *out_node = *in_node;

                match out_node.id.as_string() {
                    Ok(path) => match handle.find_node(path) {
                        Ok(cluster_data) => {
                            out_node.set_from_clam(&cluster_data);
                            return FFIError::Ok;
                        }
                        Err(e) => {
                            debug!("error {:?}", e);
                            return e;
                        }
                    },
                    Err(e) => {
                        debug!("error {:?}", e);
                        return e;
                    }
                }
            }
        }
    }
    return FFIError::NullPointerPassed;
}

#[no_mangle]
pub extern "C" fn free_string(data: *mut i8) {
    debug!("freeing string");
    helpers::free_c_char(data);
}

#[no_mangle]
pub extern "C" fn free_string_ffi(incoming: Option<&StringFFI>, outgoing: Option<&mut StringFFI>) {
    if let Some(in_data) = incoming {
        if let Some(out_data) = outgoing {
            *out_data = *in_data;
            helpers::free_string(out_data.data);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn shutdown_clam(context_ptr: OutHandlePtr) -> FFIError {
    // Handle::from_ptr(*context_ptr).self_destruct();
    // Handle::from_ptr(*context_ptr).shutdown();
    if let Some(handle) = context_ptr {
        let _ = Box::from_raw(*handle);
        return FFIError::Ok;
    } else {
        return FFIError::NullPointerPassed;
    }
}

#[no_mangle]
pub unsafe extern "C" fn init_clam(
    ptr: OutHandlePtr,
    data_name: *const u8,
    name_len: i32,
    cardinality: u32,
) -> FFIError {
    let data_name = match helpers::csharp_to_rust_utf8(data_name, name_len) {
        Ok(data_name) => data_name,
        Err(e) => {
            debug!("{:?}", e);
            return FFIError::InvalidStringPassed;
        }
    };

    match Handle::new(&data_name, cardinality as usize) {
        Ok(handle) => {
            if let Some(out_handle) = ptr {
                *out_handle = Box::into_raw(Box::new(handle));
            }

            debug!("built clam tree for {}", data_name);
            return FFIError::Ok;
        }
        Err(e) => {
            debug!("{:?}", e);
            return FFIError::HandleInitFailed;
        }
    }
}

// unsafe fn init_clam_helper(
//     data_name: &String,
//     cardinality: u32,
// ) -> Result<Rc<RefCell<Handle>>, String> {
//     // let handle = Rc::new(RefCell::new(Handle::default()));

//     // let dataset_result = handle
//     //     .as_ptr()
//     //     .as_mut()
//     //     .unwrap()
//     //     .init_dataset(data_name.as_str());

//     // if dataset_result == 0 {
//     //     debug!("failed to create dataset");
//     //     return Err(format!("failed to create dataset {}", data_name));
//     // }

//     // let root = handle
//     //     .as_ptr()
//     //     .as_mut()
//     //     .unwrap()
//     //     .build_clam(cardinality as usize);
//     // match root {
//     //     Ok(clam_root) => {
//     //         handle
//     //             .as_ptr()
//     //             .as_mut()
//     //             .unwrap()
//     //             .set_root(clam_root.clone());

//     //         return Ok(handle);
//     //     }
//     //     Err(e) => {
//     //         return Err(e);
//     //     }
//     // }
// }

#[no_mangle]
pub unsafe extern "C" fn for_each_dft(
    ptr: InHandlePtr,
    node_visitor: CBFnNodeVistor,
    start_node: *const c_char,
) -> FFIError {
    if let Some(handle) = ptr {
        if !start_node.is_null() {
            let c_str = unsafe {
                // assert!(!start_node.is_null());

                CStr::from_ptr(start_node)
            };
            let r_str = c_str.to_str().unwrap();
            debug!("start node name {}", r_str);

            // return Handle::from_ptr(ptr).for_each_dft(node_visitor, r_str.to_string());
            return handle.for_each_dft(node_visitor, r_str.to_string());
        } else {
            return FFIError::InvalidStringPassed;
        }
    }

    return FFIError::NullPointerPassed;
}

#[no_mangle]
pub extern "C" fn create_reingold_layout(
    ptr: InHandlePtr,
    node_visitor: CBFnNodeVistor,
) -> FFIError {
    if let Some(handle) = ptr {
        // return Handle::from_ptr(ptr).create_reingold_layout(node_visitor);
        return handle.create_reingold_layout(node_visitor);
    }

    return FFIError::NullPointerPassed;
}

// Option<& mut *mut Rc<RefCell<Handle>>>
// ptr: Option<&mut Rc<RefCell<Handle>>>
#[no_mangle]
pub unsafe extern "C" fn get_num_nodes(ptr: InHandlePtr) -> i32 {
    // Handle::from_ptr(ptr).get_num_nodes() + 1

    if let Some(handle) = ptr {
        return handle.get_num_nodes() + 1;
    }
    return 0;
}
