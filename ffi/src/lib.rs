mod ffi_impl;
mod utils;
// use ffi_impl::handle::Handle;

#[no_mangle]
pub extern "C" fn get_answer() -> i32 {
    42
}
