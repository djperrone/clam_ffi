//need to use partial struct to pass by reference and access sub structs
// pass by pointer with classes but cant seem to access sub structs

use std::{cell::RefCell, rc::Rc};

mod core;
mod ffi_impl;
mod utils;
use ffi_impl::{
    handle::Handle,
    node::{NodeFFI, NodeToUnity},
};
use utils::helpers;
type CBFnNodeVistor = extern "C" fn(*mut NodeFFI) -> ();

// #[no_mangle]
// pub unsafe extern "C" fn create_node_baton(data_name: *const u8, name_len: i32) -> *mut NodeFFI {
//     return Box::into_raw(Box::new(NodeFFI::default()));
// }

// #[no_mangle]
// pub unsafe extern "C" fn destroy_node_baton(node: *mut NodeFFI) -> () {
//     Box::from_raw(node).free_ids();

//     // return Box::into_raw(Box::new(NodeFFI::default()));
// }

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// A vector used in our game engine.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SuperComplexEntity {
    pub player_1: Vec3,
    pub player_2: Vec3,
    pub ammo: u64,
    /// Point to an ASCII encoded whatnot.
    pub some_str: *const u8,
    pub str_len: u32,
}

#[no_mangle]
pub unsafe extern "C" fn example_double_super_complex_entity(
    context: Option<&mut Handle>,
    incoming: *mut NodeToUnity,
    outgoing: *mut NodeToUnity,
) -> () {
    let _context = context.unwrap();
    // let incoming = incoming.unwrap();
    // let outgoing = outgoing.unwrap();

    // *outgoing = *incoming;

    (*incoming).cardinality = 2;
    (*incoming).arg_center = 2;
    (*incoming).arg_radius = 2;
    (*incoming).depth = 2;
    debug!("card ouy {}", (*incoming).cardinality);
    debug!("card root {}", (*incoming).cardinality);
    debug!("finished passing data");
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
