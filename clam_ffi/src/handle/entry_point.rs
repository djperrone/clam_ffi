use crate::handle::handle::Handle;
use crate::utils::distances::DistanceMetric;
use crate::utils::helpers;
use crate::utils::types::{InHandlePtr, OutHandlePtr};

use crate::utils::error::FFIError;

use crate::debug;

pub unsafe fn shutdown_clam_impl(context_ptr: OutHandlePtr) -> FFIError {
    if let Some(handle) = context_ptr {
        let _ = Box::from_raw(*handle);
        return FFIError::Ok;
    } else {
        debug!("shtudown clam handle not valid?");
        return FFIError::NullPointerPassed;
    }
}

pub unsafe fn force_physics_shutdown(ptr: InHandlePtr) -> i32 {
    // Handle::from_ptr(ptr).get_num_nodes() + 1

    if let Some(handle) = ptr {
        // debug!("cardinality: {}", handle.tree_height() + 1);
        handle.force_physics_shutdown();

        // return handle.tree_height() + 1;
    }
    debug!("handle not created");

    return 0;
}

pub unsafe fn init_clam_impl(
    ptr: OutHandlePtr,
    data_name: *const u8,
    name_len: i32,
    cardinality: u32,
    distance_metric: DistanceMetric,

) -> FFIError {
    let data_name = match helpers::csharp_to_rust_utf8(data_name, name_len) {
        Ok(data_name) => data_name,
        Err(e) => {
            debug!("{:?}", e);
            return FFIError::InvalidStringPassed;
        }
    };

    match Handle::new(&data_name, cardinality as usize, distance_metric) {
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
