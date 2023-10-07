//need to use partial struct to pass by reference and access sub structs
// pass by pointer with classes but cant seem to access sub structs

use std::ffi::c_char;
mod ffi_impl;
mod graph;
mod handle;
mod tests;
mod tree_layout;
mod utils;
use ffi_impl::{
    cluster_data::ClusterData,
    lib_impl::{
        color_by_dist_to_query_impl, distance_to_other_impl, for_each_dft_impl,
        get_cluster_data_impl, test_cakes_rnn_query_impl, tree_height_impl,
    },
};
use graph::entry::{
    physics_update_async_impl, run_force_directed_graph_sim_impl, shutdown_physics_impl,
};
use tree_layout::entry_point::{draw_heirarchy_impl, draw_heirarchy_offset_from_impl};
use utils::{
    debug,
    error::FFIError,
    types::{InHandlePtr, OutHandlePtr},
};

use crate::handle::entry_point::{init_clam_impl, shutdown_clam_impl};

type CBFnNodeVisitor = extern "C" fn(Option<&ClusterData>) -> ();

// ------------------------------------- Startup/Shutdown -------------------------------------

#[no_mangle]
pub unsafe extern "C" fn init_clam(
    ptr: OutHandlePtr,
    data_name: *const u8,
    name_len: i32,
    cardinality: u32,
) -> FFIError {
    return init_clam_impl(ptr, data_name, name_len, cardinality);
}

#[no_mangle]
pub unsafe extern "C" fn shutdown_clam(context_ptr: OutHandlePtr) -> FFIError {
    return shutdown_clam_impl(context_ptr);
}

// -------------------------------------  Tree helpers -------------------------------------

#[no_mangle]
pub unsafe extern "C" fn for_each_dft(
    ptr: InHandlePtr,
    node_visitor: CBFnNodeVisitor,
    start_node: *const c_char,
) -> FFIError {
    return for_each_dft_impl(ptr, node_visitor, start_node);
}

#[no_mangle]
pub unsafe extern "C" fn tree_height(ptr: InHandlePtr) -> i32 {
    return tree_height_impl(ptr);
}

// ------------------------------------- Cluster Helpers -------------------------------------

#[no_mangle]
pub unsafe extern "C" fn get_cluster_data(
    context: InHandlePtr,
    incoming: Option<&ClusterData>,
    outgoing: Option<&mut ClusterData>,
) -> FFIError {
    return get_cluster_data_impl(context, incoming, outgoing);
}

#[no_mangle]
pub unsafe extern "C" fn distance_to_other(
    ptr: InHandlePtr,
    node_name1: *const c_char,
    node_name2: *const c_char,
) -> f32 {
    return distance_to_other_impl(ptr, node_name1, node_name2);
}

// ------------------------------------- Reingold Tilford Tree Layout -------------------------------------

#[no_mangle]
pub extern "C" fn draw_heirarchy(ptr: InHandlePtr, node_visitor: CBFnNodeVisitor) -> FFIError {
    return draw_heirarchy_impl(ptr, node_visitor);
}

#[no_mangle]
pub unsafe extern "C" fn draw_heirarchy_offset_from(
    ptr: InHandlePtr,
    root: Option<&ClusterData>,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    return draw_heirarchy_offset_from_impl(ptr, root, node_visitor);
}

// ------------------------------------- Graph Physics -------------------------------------

#[no_mangle]
pub unsafe extern "C" fn run_force_directed_graph_sim(
    context: InHandlePtr,
    arr_ptr: *mut ClusterData,
    len: i32,
    scalar: f32,
    max_iters: i32,
    edge_detect_cb: CBFnNodeVisitor,
    // physics_update_cb: CBFnNodeVisitor,
) -> FFIError {
    return run_force_directed_graph_sim_impl(
        context,
        arr_ptr,
        len,
        scalar,
        max_iters,
        edge_detect_cb,
    );
}

#[no_mangle]
pub unsafe extern "C" fn physics_update_async(
    context: InHandlePtr,
    updater: CBFnNodeVisitor,
) -> FFIError {
    return physics_update_async_impl(context, updater);
}

#[no_mangle]
pub extern "C" fn shutdown_physics(ptr: InHandlePtr) -> FFIError {
    return shutdown_physics_impl(ptr);
}

// ------------------------------------- RNN Search -------------------------------------

#[no_mangle]
pub unsafe extern "C" fn test_cakes_rnn_query(
    ptr: InHandlePtr,
    search_radius: f32,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    return test_cakes_rnn_query_impl(ptr, search_radius, node_visitor);
}

#[no_mangle]
pub unsafe extern "C" fn color_by_dist_to_query(
    context: InHandlePtr,
    arr_ptr: *mut ClusterData,
    len: i32,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    return color_by_dist_to_query_impl(context, arr_ptr, len, node_visitor);
}
