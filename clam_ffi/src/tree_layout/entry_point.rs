use crate::{
    ffi_impl::cluster_data::ClusterData,
    utils::{error::FFIError, types::InHandlePtr},
    CBFnNodeVisitor,
};

pub fn draw_heirarchy_impl(ptr: InHandlePtr, node_visitor: CBFnNodeVisitor) -> FFIError {
    if let Some(handle) = ptr {
        // return Handle::from_ptr(ptr).create_reingold_layout(node_visitor);
        return handle.create_reingold_layout(node_visitor);
    }

    return FFIError::NullPointerPassed;
}

pub unsafe fn draw_heirarchy_offset_from_impl(
    ptr: InHandlePtr,
    root: Option<&ClusterData>,
    current_depth: i32,
    max_depth: i32,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    if let Some(handle) = ptr {
        if let Some(node) = root {
            return handle.create_reingold_layout_offset_from(
                node,
                current_depth,
                max_depth,
                node_visitor,
            );
        }
        // return Handle::from_ptr(ptr).create_reingold_layout(node_visitor);
    }

    return FFIError::NullPointerPassed;
}
