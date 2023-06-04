// extern crate clam_ffi;
// extern crate clam_ffi;
// use clam::core::cluster;
// use clam_ffi::get_answer;
// use clam_ffi::*;
use interoptopus::{ffi_function, ffi_type, function, Inventory, InventoryBuilder};
/// A simple type in our FFI layer.
#[ffi_type]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// Function using the type.
#[ffi_function]
#[no_mangle]
pub extern "C" fn my_function(input: Vec2) -> Vec2 {
    input
}

// This will create a function `my_inventory` which can produce
// an abstract FFI representation (called `Library`) for this crate.
pub fn my_inventory() -> Inventory {
    {
        InventoryBuilder::new()
            .register(function!(my_function))
            // .register(function!(get_answer))
            .inventory()
    }
}
