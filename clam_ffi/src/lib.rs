mod ffi_impl;
mod utils;
use std::borrow::BorrowMut;
use std::ffi::c_char;
use std::ffi::CStr;

use clam::core::cluster::Cluster;
use clam::core::cluster_criteria::PartitionCriteria;
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
    name_len: i32,
    cardinality: u32,
) -> u32 {
    let data_name = match csharp_to_rust_utf8(data_name, name_len) {
        Ok(data_name) => data_name,
        Err(e) => {
            debug!("{}", e);
            return 0;
        }
    };
    let mut handle = Box::from_raw(ptr as *mut Handle);
    // *handle = Handle::default();
    // let result = Handle::init(&mut *handle, data_name.as_str(), cardinality as usize);
    let result = handle.init_dataset(data_name.as_str());
    if result == 0 {
        debug!("failed to create dataset");
        return 0;
    }
    handle.build_clam(cardinality as usize);
    return 1;
    // let root = Handle::build_clam(handle.get_dataset(), cardinality as usize);
    // handle.set_root(root);
    // Handle::build_clam_root(handle.get_dataset(), handle.get_root_mut(), cardinality as usize);

    // if result == 0 {
    //     debug!("error building root");
    //     return 0;
    // }
    // handle.build_clam_root(cardinality as usize);

    // let root = {
    //     let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality as usize);
    // let dataset = handle.get_dataset().as_ref().unwrap();
    //     // self.clam_root = None;
    // let root = Cluster::new_root(dataset)
    //     .partition(&criteria, true)
    //     .with_seed(0);

    //     root
    // };

    // handle.set_root(root);
    // std::mem::forget(handle);
    // handle.to_ptr();
    // let mut handle = Handle::default();
    // let result = Handle::init(&mut handle, data_name.as_str(), cardinality as usize);
    // let test = Box::new(handle);
    // *ptr = Box::into_raw(test) as *mut Handle;
    // *ptr = handle.to_ptr();

    // debug!("creating tree {}", data_name);
    // *ptr = Box::into_raw(Box::new(Handle::default()));
    // let handle = Box::from_raw(ptr);
    // let mut handle = Handle::default();
    // let t = Box::new(Handle::init(
    //     &mut handle.as,
    //     data_name.as_str(),
    //     cardinality as usize,
    // ));

    // let result = Handle::init(&mut handle, data_name.as_str(), cardinality as usize);
    // let handle_box = Box::new(handle);
    // *ptr = Box::into_raw(handle_box) as *mut Handle;
    // debug!("finished creating tree");

    // let len = match t.get_layout() {
    //     Some(pos_layout) => pos_layout.len(),
    //     None => 0,
    // };

    // unsafe {
    //     *ptr = t.to_ptr();
    // }

    // return 0 as u32;
}

pub unsafe fn csharp_to_rust_utf8(utf8_str: *const u8, utf8_len: i32) -> Result<String, String> {
    let slice = std::slice::from_raw_parts(utf8_str, utf8_len as usize);
    match String::from_utf8(slice.to_vec()) {
        Ok(str) => Ok(str),
        Err(e) => Err(String::from("invalid csharp_to_rust_utf8 conversion")),
    }
}
