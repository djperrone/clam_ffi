//need to use partial struct to pass by reference and access sub structs
// pass by pointer with classes but cant seem to access sub structs

use std::{
    cell::RefCell,
    ffi::{c_char, CStr},
    rc::Rc,
};

mod core;
mod ffi_impl;
mod utils;
use ffi_impl::{handle::Handle, node::NodeFFI};
use utils::helpers;
type CBFnNodeVistor = extern "C" fn(*mut NodeFFI) -> ();

#[repr(C)]
#[derive(Clone, Debug)]
pub struct ComplexStruct {
    my_str: StringStruct1,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct StringStruct1 {
    pub utf8_str: *mut u8,
    pub utf8_len: i32,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct StringStruct2 {
    pub s: String,
}

impl StringStruct2 {
    pub unsafe fn new(other: &StringStruct1) -> Self {
        StringStruct2 {
            s: helpers::csharp_to_rust_utf8(other.utf8_str, other.utf8_len)
                .unwrap_or("failed to do stuff".to_string()),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_string_struct2(
    incoming: Option<&ComplexStruct>,
    outgoing: Option<&mut ComplexStruct>,
) -> () {
    if let Some(in_struct) = incoming {
        let some_str =
            helpers::csharp_to_rust_utf8(in_struct.my_str.utf8_str, in_struct.my_str.utf8_len);

        // debug!("start string struct test ");

        // let ss = StringStruct2::new(in_struct);
        let tests = "test".to_string();
        let mut test = *in_struct.my_str.utf8_str;
        *(in_struct.my_str.utf8_str.add(1)) = 107;
        let mut i = 0 as usize;
        for ch in tests.chars() {
            *(in_struct.my_str.utf8_str.add(i as usize)) = ch as u8;
            i += 1;
        }
        debug!("string struct test {:?}", some_str);
        debug!("string struct test {:?}", *in_struct.my_str.utf8_str)
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_string_struct(
    incoming: Option<&StringStruct1>,
    outgoing: Option<&mut StringStruct1>,
) -> () {
    if let Some(in_struct) = incoming {
        let some_str = helpers::csharp_to_rust_utf8(in_struct.utf8_str, in_struct.utf8_len);

        // debug!("start string struct test ");

        // let ss = StringStruct2::new(in_struct);
        let mut test = *in_struct.utf8_str;
        *in_struct.utf8_str = 109;
        debug!("string struct test {:?}", some_str);
        debug!("string struct test {:?}", *in_struct.utf8_str)
    }
}

#[no_mangle]
pub extern "C" fn test_string_fn(s: *const c_char) -> u32 {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };
    debug!("cstr testing {:?}", c_str);
    let r_str = c_str.to_str().unwrap();
    r_str.chars().count() as u32
}

#[no_mangle]
pub unsafe extern "C" fn get_node_data(
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
