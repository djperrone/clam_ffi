use crate::{
    debug,
    ffi_impl::cluster_data::ClusterData,
    utils::{error::FFIError, types::InHandlePtr},
    CBFnNodeVisitor, CBFnNodeVisitorMut,
};

use super::graph_builder;

pub unsafe fn physics_update_async_impl(
    context: InHandlePtr,
    updater: CBFnNodeVisitor,
) -> FFIError {
    if let Some(handle) = context {
        // debug!("calling physics update async");
        let err = handle.physics_update_async(updater);
        // debug!("physics update result {:?}", err);
        return err;
    } else {
        return FFIError::NullPointerPassed;
    }
}

pub unsafe fn init_force_directed_graph_impl(
    context: InHandlePtr,
    arr_ptr: *mut ClusterData,
    len: i32,
    scalar: f32,
    max_iters: i32,
    // edge_detect_cb: CBFnNodeVisitorMut,
    // physics_update_cb: CBFnNodeVisitor,
) -> FFIError {
    if let Some(handle) = context {
        if arr_ptr.is_null() {
            return FFIError::NullPointerPassed;
        }
        let arr = std::slice::from_raw_parts_mut(arr_ptr, len as usize);

        // return handle.second_build_graph(arr, scalar, max_iters, edge_detect_cb, physics_update_cb);

        match graph_builder::build_force_directed_graph(
            arr, handle, scalar,
            max_iters,
            // edge_detect_cb,
            // physics_update_cb,
        ) {
            Ok(g) => {
                handle.set_graph(g);
                // handle.init_unity_edges(edge_detect_cb);
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
pub unsafe fn init_graph_vertices_impl(
    context: InHandlePtr,
    // arr_ptr: *mut ClusterData,
    // len: i32,
    // scalar: f32,
    // max_iters: i32,
    edge_detect_cb: CBFnNodeVisitorMut,
    // physics_update_cb: CBFnNodeVisitor,
) -> FFIError {
    if let Some(handle) = context {
        // if arr_ptr.is_null() {
        //     return FFIError::NullPointerPassed;
        // }

        // if let Some(graph) = handle.

        // let arr = std::slice::from_raw_parts_mut(arr_ptr, len as usize);

        return handle.init_unity_edges(edge_detect_cb);

        // match graph_builder::build_force_directed_graph(
        //     arr,
        //     handle,
        //     scalar,
        //     max_iters,
        //     edge_detect_cb,
        //     // physics_update_cb,
        // ) {
        //     Ok(g) => {
        //         handle.set_graph(g);
        //         return FFIError::Ok;
        //     }
        //     Err(e) => {
        //         debug!("launch thread result {:?}", e);
        // return e;
        // }
        // }
        // } else {
        //     return FFIError::NullPointerPassed;
    }
    return FFIError::Ok;
}

pub fn shutdown_physics_impl(ptr: InHandlePtr) -> FFIError {
    if let Some(handle) = ptr {
        // return Handle::from_ptr(ptr).create_reingold_layout(node_visitor);
        return handle.shutdown_physics();
    }

    return FFIError::NullPointerPassed;
}

pub fn get_num_edges_in_graph_impl(ptr: InHandlePtr) -> i32 {
    if let Some(handle) = ptr {
        // return Handle::from_ptr(ptr).create_reingold_layout(node_visitor);
        return handle.get_num_edges_in_graph();
    }
    return -1;
    // return FFIError::NullPointerPassed;
}
