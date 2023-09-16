use crate::{
    ffi_impl::node::NodeData,
    utils::{
        error::FFIError,
        types::{Clusterf32, InHandlePtr},
    },
    CBFnNodeVisitor,
};

use super::reingold_impl;

// pub fn create_layout() -> () {}

#[no_mangle]
pub extern "C" fn draw_heirarchy(ptr: InHandlePtr, node_visitor: CBFnNodeVisitor) -> FFIError {
    if let Some(handle) = ptr {
        // return Handle::from_ptr(ptr).create_reingold_layout(node_visitor);
        return handle.create_reingold_layout(node_visitor);
    }

    return FFIError::NullPointerPassed;
}

pub fn run(
    clam_root: &Clusterf32,
    labels: &Option<Vec<u8>>,
    node_visitor: crate::CBFnNodeVisitor,
) -> FFIError {
    // if let Some(labels) = &self.labels {
    let layout_root = reingold_impl::Node::init_draw_tree(clam_root, labels);
    let result = reingoldify(layout_root, node_visitor);

    return result;
}

fn reingoldify(root: reingold_impl::Link, node_visitor: crate::CBFnNodeVisitor) -> FFIError {
    if let Some(_) = root.clone() {
        reingoldify_helper(root.clone(), node_visitor);

        return FFIError::Ok;
    }
    return FFIError::NullPointerPassed;
}

fn reingoldify_helper(root: reingold_impl::Link, node_visitor: crate::CBFnNodeVisitor) -> () {
    if let Some(node) = root {
        let mut baton = NodeData::from_reingold_node(&node.as_ref().borrow());

        node_visitor(Some(&baton));
        baton.free_ids();

        reingoldify_helper(node.as_ref().borrow().get_left_child(), node_visitor);
        reingoldify_helper(node.as_ref().borrow().get_right_child(), node_visitor);
    }
}
