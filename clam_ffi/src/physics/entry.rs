use crate::{
    debug,
    ffi_impl::node::NodeData,
    utils::{error::FFIError, types::InHandlePtr},
    CBFnNodeVisitor,
};

use super::graph_builder;

#[no_mangle]
pub unsafe extern "C" fn physics_update_async(
    context: InHandlePtr,
    updater: CBFnNodeVisitor,
) -> FFIError {
    if let Some(handle) = context {
        // debug!("calling physics update async");
        let err = handle.physics_update_async(updater);
        debug!("physics update result {:?}", err);
        return err;
    } else {
        return FFIError::NullPointerPassed;
    }
    // return FFIError::Ok;
}

#[no_mangle]
pub unsafe extern "C" fn run_force_directed_graph_sim(
    context: InHandlePtr,
    arr_ptr: *mut NodeData,
    len: i32,
    scalar: f32,
    max_iters: i32,
    edge_detect_cb: CBFnNodeVisitor,
    // physics_update_cb: CBFnNodeVisitor,
) -> FFIError {
    if let Some(handle) = context {
        if arr_ptr.is_null() {
            return FFIError::NullPointerPassed;
        }
        let arr = std::slice::from_raw_parts_mut(arr_ptr, len as usize);

        // return handle.second_build_graph(arr, scalar, max_iters, edge_detect_cb, physics_update_cb);

        match graph_builder::build_force_directed_graph(
            arr,
            handle,
            scalar,
            max_iters,
            edge_detect_cb,
            // physics_update_cb,
        ) {
            Ok(g) => {
                handle.set_graph(g);
                return FFIError::Ok;
            }
            Err(e) => {
                debug!("launch thread result {:?}", e);
                return e;
            }
        }
    } else {
        return FFIError::NullPointerPassed;
    }
    // return FFIError::Ok;
}

#[no_mangle]
pub extern "C" fn shutdown_physics(ptr: InHandlePtr) -> FFIError {
    if let Some(handle) = ptr {
        // return Handle::from_ptr(ptr).create_reingold_layout(node_visitor);
        return handle.shutdown_physics();
    }

    return FFIError::NullPointerPassed;
}
