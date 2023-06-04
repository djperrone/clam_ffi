mod ffi_impl;
mod utils;
// use ffi_impl::handle::Handle;
use interoptopus::{ffi_function, ffi_type, function, Inventory, InventoryBuilder};

#[ffi_function]
#[no_mangle]
pub extern "C" fn get_answer() -> i32 {
    42
}
