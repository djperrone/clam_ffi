use crate::debug;

#[no_mangle]
pub unsafe extern "C" fn hello_world() {
    debug!("hello world!");
}

#[no_mangle]
pub unsafe extern "C" fn test() {
    debug!("hello yrdsy!");
}
