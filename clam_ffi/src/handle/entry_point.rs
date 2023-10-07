use crate::handle::handle::Handle;
use crate::utils::helpers;
use crate::utils::types::OutHandlePtr;

use crate::utils::error::FFIError;

use crate::debug;

pub unsafe fn shutdown_clam_impl(context_ptr: OutHandlePtr) -> FFIError {
    if let Some(handle) = context_ptr {
        let _ = Box::from_raw(*handle);
        return FFIError::Ok;
    } else {
        return FFIError::NullPointerPassed;
    }
}

pub unsafe fn init_clam_impl(
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
