//need to use partial struct to pass by reference and access sub structs
// pass by pointer with classes but cant seem to access sub structs

use std::{
    cell::RefCell,
    ffi::{c_char, CStr, CString},
    rc::Rc,
};
mod core;
mod ffi_impl;
mod tests;
mod utils;
use ffi_impl::{
    handle::{Clusterf32, Handle},
    node::{NodeData, StringFFI},
};
use utils::helpers;
type CBFnNodeVistor2 = extern "C" fn(Option<&NodeData>) -> ();

// #[no_mangle]
// pub unsafe extern "C" fn find_node(
//     context: Option<&mut Handle>,
//     incoming: Option<&NodeData>,
//     outgoing: Option<&mut NodeData>,
// ) -> bool {
//     if let Some(handle) = context {
//         if let Some(in_node) = incoming {
//             if let Some(mut out_node) = outgoing {
//                 *out_node = *in_node;

//                 match handle.find_node(out_node.id.as_string().unwrap()) {
//                     Ok(mut data) => {
//                         out_node.cardinality = data.cardinality;
//                         out_node.arg_center = data.arg_center;
//                         out_node.arg_radius = data.arg_radius;
//                         out_node.depth = data.depth;
//                         data.free_ids();
//                         return true;
//                     }
//                     Err(e) => {
//                         debug!("{}", e);
//                         return false;
//                     }
//                 }
//             }
//         }
//     }
//     debug!("could not find node {}");
//     return false;
// }

#[no_mangle]
pub unsafe extern "C" fn get_cluster_data(
    context: Option<&mut Handle>,
    incoming: Option<&NodeData>,
    outgoing: Option<&mut NodeData>,
) -> bool {
    if let Some(handle) = context {
        if let Some(in_node) = incoming {
            if let Some(mut out_node) = outgoing {
                *out_node = *in_node;

                match out_node.id.as_string() {
                    Ok(path) => match handle.find_node(path) {
                        Ok(cluster_data) => {
                            // ---- set from clam ---
                            // out_node.cardinality = cluster_data.cardinality() as i32;
                            // out_node.arg_center = cluster_data.arg_center() as i32;
                            // out_node.arg_radius = cluster_data.arg_radius() as i32;
                            // out_node.depth = cluster_data.depth() as i32;
                            out_node.set_from_clam(&cluster_data);
                        }
                        Err(e) => {
                            debug!("{}", e);
                            return false;
                        }
                    },
                    Err(e) => {
                        debug!("{}", e);
                        return false;
                    }
                }
            }
        }
    }
    return false;
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
pub unsafe extern "C" fn init_clam(
    ptr: &mut *mut Handle,
    data_name: *const u8,
    name_len: i32,
    cardinality: u32,
) -> i32 {
    let data_name = match helpers::csharp_to_rust_utf8(data_name, name_len) {
        Ok(data_name) => data_name,
        Err(e) => {
            debug!("{}", e);
            return 0;
        }
    };

    match init_clam_helper(&data_name, cardinality) {
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

unsafe fn init_clam_helper<'a>(
    data_name: &String,
    cardinality: u32,
) -> Result<Rc<RefCell<Handle<'a>>>, String> {
    let handle = Rc::new(RefCell::new(Handle::default()));

    let dataset_result = handle
        .as_ptr()
        .as_mut()
        .unwrap()
        .init_dataset(data_name.as_str());

    if dataset_result == 0 {
        debug!("failed to create dataset");
        return Err(format!("failed to create dataset {}", data_name));
    }

    let root = handle
        .as_ptr()
        .as_mut()
        .unwrap()
        .build_clam(cardinality as usize);
    match root {
        Ok(clam_root) => {
            handle
                .as_ptr()
                .as_mut()
                .unwrap()
                .set_root(clam_root.clone());

            return Ok(handle);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

#[no_mangle]
pub extern "C" fn for_each_dft(
    ptr: *mut Handle,
    node_visitor: CBFnNodeVistor2,
    start_node: *const c_char,
) -> i32 {
    if !ptr.is_null() {
        if !start_node.is_null() {
            let c_str = unsafe {
                // assert!(!start_node.is_null());

                CStr::from_ptr(start_node)
            };
            let r_str = c_str.to_str().unwrap();

            return Handle::from_ptr(ptr).for_each_dft(node_visitor, r_str.to_string());
        }
    }

    return 0;
}

#[no_mangle]
pub extern "C" fn create_reingold_layout(ptr: *mut Handle, node_visitor: CBFnNodeVistor2) -> i32 {
    if !ptr.is_null() {
        return Handle::from_ptr(ptr).create_reingold_layout(node_visitor);
    }

    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn get_num_nodes(ptr: *mut Handle) -> i32 {
    Handle::from_ptr(ptr).get_num_nodes() + 1
}
