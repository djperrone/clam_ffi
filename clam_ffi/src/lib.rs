//need to use partial struct to pass by reference and access sub structs
// pass by pointer with classes but cant seem to access sub structs

use std::{cell::RefCell, ffi::CString, rc::Rc};

mod core;
mod ffi_impl;
mod utils;
use ffi_impl::{
    handle::Handle,
    node::{NodeFFI, NodeToUnity},
};
use utils::helpers;
type CBFnNodeVistor = extern "C" fn(*mut NodeFFI) -> ();

#[no_mangle]
pub extern "C" fn get_node_data2(
    context: Option<&mut Handle>,
    incoming: Option<&NodeFFI>,
    outgoing: Option<&mut NodeFFI>,
) -> () {
    if let Some(handle) = context {
        if let Some(in_node) = incoming {
            if let Some(out_node) = outgoing {
                *out_node = *in_node;

                if let Some(root) = handle.get_root() {
                    unsafe {
                        let test_name =
                            helpers::csharp_to_rust_utf8(out_node.id as *const u8, out_node.id_len);
                        match test_name {
                            Ok(name) => {
                                debug!("test name -- {} -- worked!", name);
                            }
                            Err(e) => {
                                debug!("test name failed! {}", e);
                            }
                        }
                    }
                    out_node.set_from_clam_node(root.clone());
                    debug!("node data card {}", out_node.cardinality);
                    debug!("node data card {}", out_node.arg_center);
                    debug!("node data card {}", out_node.arg_radius);
                    debug!("node data card {}", out_node.depth);
                    debug!("node data card {:?}", out_node.pos);
                    debug!("node data card {:?}", out_node.color);
                    // debug!("node data card {}", out_node.id);
                    return;
                }
            }
        }
    }
    debug!("get_node data2 went wrong");
}

#[no_mangle]
pub unsafe extern "C" fn get_node_data3(
    context: Option<&mut Handle>,
    data_name: *const u8,
    name_len: i32,
    incoming: Option<&NodeFFI>,
    outgoing: Option<&mut NodeFFI>,
) -> () {
    if let Some(handle) = context {
        if let Some(in_node) = incoming {
            if let Some(out_node) = outgoing {
                let data_name = match helpers::csharp_to_rust_utf8(data_name, name_len) {
                    Ok(data_name) => data_name,
                    Err(e) => {
                        debug!("{}", e);
                        return;
                    }
                };
                // Node::NodeData node_data
                *out_node = *in_node;

                match handle.get_node_data(data_name.chars().rev().collect()) {
                    Ok(data) => {
                        // unsafe {
                        //     let test_name =
                        //         helpers::csharp_to_rust_utf8(out_node.id as *const u8, out_node.id_len);
                        //     match test_name {
                        //         Ok(name) => {
                        //             debug!("test name -- {} -- worked!", name);
                        //         }
                        //         Err(e) => {
                        //             debug!("test name failed! {}", e);
                        //         }
                        //     }
                        // }
                        out_node.set_from_node_ffi(&data);
                      
                        debug!("node data card {}", out_node.cardinality);
                        debug!("node data card {}", out_node.arg_center);
                        debug!("node data card {}", out_node.arg_radius);
                        debug!("node data card {}", out_node.depth);
                        debug!("node data card {:?}", out_node.pos);
                        debug!("node data card {:?}", out_node.color);
                        // debug!("node data card {}", out_node.id);
                        return;
                    }
                    Err(e) => {
                        debug!("{}", e)
                    }
                }
                debug!("get_node data3 went wrong1");
                return;
            }

            debug!("get_node data3 went wrong2");
            return;
        }
        debug!("get_node data3 went wrong3");
        return;
    }
    debug!("get_node data3 went wrong4");
    return;
}

#[no_mangle]
pub extern "C" fn free_node_string(
    context: Option<&mut Handle>,
    incoming: Option<&NodeFFI>,
    outgoing: Option<&mut NodeFFI>,
) -> () {
    if let Some(handle) = context {
        if let Some(in_node) = incoming {
            if let Some(out_node) = outgoing {
                *out_node = *in_node;

                if let Some(_) = handle.get_root() {
                    out_node.free_ids();
                    debug!("freed ids");
                    return;
                }
            }
        }
    }
    debug!("free_node_string we nt wrong");
}
#[no_mangle]
pub extern "C" fn free_string(data: *mut i8) {
    debug!("freeing string");
    helpers::free_c_char(data);
}

#[no_mangle]
pub extern "C" fn get_node_data(
    context: *mut Handle,
    incoming: Option<&NodeToUnity>,
    outgoing: Option<&mut NodeToUnity>,
) -> () {
    // if let Some(context) = context {
    if let Some(in_node) = incoming {
        if let Some(out_node) = outgoing {
            *out_node = *in_node;
            if let Some(node) = Handle::from_ptr(context).get_root() {
                let node = node.as_ref().borrow();
                out_node.arg_center = node.arg_center() as i32;
                out_node.arg_radius = node.arg_radius() as i32;
                out_node.cardinality = node.cardinality() as i32;
                out_node.depth = node.depth() as i32;
                debug!("card ouy {}", out_node.cardinality);
                debug!("card root {}", node.cardinality());
                debug!("finished passing data");
                return;
            }
        }
    }
    // }

    debug!("somemthing went wrong ")
    // // let _context = context.unwrap();
    // let incoming = incoming.unwrap();
    // let outgoing = outgoing.unwrap();

    // outgoing.ammo *= 2;
    // outgoing.player_1.x *= 2.0;
    // outgoing.player_1.y *= 2.0;
    // outgoing.player_1.z *= 2.0;
    // outgoing.player_2.x *= 2.0;
    // outgoing.player_2.y *= 2.0;
    // outgoing.player_2.z *= 2.0;
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

    // hello_world();

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
pub unsafe extern "C" fn traverse_tree_df(ptr: *mut Handle, node_visitor: CBFnNodeVistor) -> i32 {
    if !ptr.is_null() {
        return Handle::from_ptr(ptr).traverse_tree_df(node_visitor);
    }

    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn create_reingold_layout(
    ptr: *mut Handle,
    node_visitor: CBFnNodeVistor,
) -> i32 {
    if !ptr.is_null() {
        return Handle::from_ptr(ptr).create_reingold_layout(node_visitor);
    }

    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn get_num_nodes(ptr: *mut Handle) -> i32 {
    Handle::from_ptr(ptr).get_num_nodes() + 1
}
