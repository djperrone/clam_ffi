//need to use partial struct to pass by reference and access sub structs
// pass by pointer with classes but cant seem to access sub structs

use std::ffi::{c_char, CStr};
mod core;
mod ffi_impl;
mod physics;
mod tests;
mod tree_layout;
mod utils;
use ffi_impl::{
    cluster_data::{ClusterData, StringFFI},
    cluster_data_wrapper::ClusterDataWrapper,
};
use utils::{debug, error::FFIError, helpers, types::InHandlePtr};

use crate::utils::types::Clusterf32;

type CBFnNodeVisitor = extern "C" fn(Option<&ClusterData>) -> ();

// #[no_mangle]
// pub unsafe extern "C" fn init_force_directed_sim(
//     context: InHandlePtr,
//     arr_ptr: *mut NodeData,
//     len: i32,
//     node_visitor: CBFnNodeVisitor,
// ) -> FFIError {
//     if let Some(handle) = context {
//         if arr_ptr.is_null() {
//             return FFIError::NullPointerPassed;
//         }
//         let arr = std::slice::from_raw_parts_mut(arr_ptr, len as usize);

//         let err = handle.init_force_directed_sim(arr, node_visitor);
//         debug!("init graph result {:?}", err);
//         return err;
//     } else {
//         return FFIError::NullPointerPassed;
//     }
//     // return FFIError::Ok;
// }

// #[no_mangle]
// pub unsafe extern "C" fn launch_physics_thread(
//     context: InHandlePtr,
//     arr_ptr: *mut NodeData,
//     len: i32,
//     scalar: f32,
//     max_iters: i32,
//     edge_detect_cb: CBFnNodeVisitor,
//     // physics_update_cb: CBFnNodeVisitor,
// ) -> FFIError {
//     if let Some(handle) = context {
//         if arr_ptr.is_null() {
//             return FFIError::NullPointerPassed;
//         }
//         let arr = std::slice::from_raw_parts_mut(arr_ptr, len as usize);

//         let err = handle.build_force_directed_graph(
//             arr,
//             scalar,
//             max_iters,
//             edge_detect_cb,
//             // physics_update_cb,
//         );
//         debug!("launch thread result {:?}", err);
//         return err;
//     } else {
//         return FFIError::NullPointerPassed;
//     }
//     // return FFIError::Ok;
// }

#[no_mangle]
pub unsafe extern "C" fn color_by_dist_to_query(
    context: InHandlePtr,
    arr_ptr: *mut ClusterData,
    len: i32,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    if let Some(handle) = context {
        if arr_ptr.is_null() {
            return FFIError::NullPointerPassed;
        }
        debug!("creating string arr");
        let arr = std::slice::from_raw_parts(arr_ptr, len as usize);

        let mut ids = Vec::new();
        for node in arr {
            ids.push(node.id.as_string().unwrap());
        }

        let err = handle.color_by_dist_to_query(ids.as_slice(), node_visitor);
        debug!("color result {:?}", err);
        return err;
    } else {
        return FFIError::NullPointerPassed;
    }
    // return FFIError::Ok;
}

// #[no_mangle]
// pub unsafe extern "C" fn detect_edges(
//     context: InHandlePtr,
//     arr_ptr: *mut NodeData,
//     len: i32,
//     node_visitor: CBFnNodeVisitor,
// ) -> FFIError {
//     if let Some(handle) = context {
//         if arr_ptr.is_null() {
//             return FFIError::NullPointerPassed;
//         }
//         debug!("creating string arr");
//         let arr = std::slice::from_raw_parts(arr_ptr, len as usize);

//         // let mut ids = Vec::new();
//         // for node in arr {
//         //     ids.push(node.id.as_string().unwrap());
//         // }
//         let mut clusters: Vec<&Clusterf32> = Vec::new();

//         for c in arr {
//             if let Ok(cluster) = handle.find_node(c.id.as_string().unwrap()) {
//                 clusters.push(cluster);
//             }
//         }
//         let err = handle.detect_edges(&clusters, node_visitor);
//         debug!("color result {:?}", err);
//         return FFIError::Ok;
//     } else {
//         return FFIError::NullPointerPassed;
//     }
//     // return FFIError::Ok;
// }

// #[no_mangle]
// pub unsafe extern "C" fn apply_forces(
//     context: InHandlePtr,
//     scalar: f32,

//     node_visitor: CBFnNodeVisitor,
// ) -> FFIError {
//     if let Some(handle) = context {
//         let err = handle.apply_forces(node_visitor, scalar);

//         return err;
//     } else {
//         return FFIError::NullPointerPassed;
//     }
//     // return FFIError::Ok;
// }

#[no_mangle]
pub unsafe extern "C" fn get_cluster_data(
    context: InHandlePtr,
    incoming: Option<&ClusterData>,
    outgoing: Option<&mut ClusterData>,
) -> FFIError {
    if let Some(handle) = context {
        if let Some(in_node) = incoming {
            if let Some(out_node) = outgoing {
                *out_node = *in_node;

                match out_node.id.as_string() {
                    Ok(path) => match handle.find_node(path) {
                        Ok(cluster_data) => {
                            out_node.set_from_clam(&cluster_data);
                            if let Some(query) = handle.get_current_query() {
                                out_node.dist_to_query = cluster_data
                                    .distance_to_instance(handle.data().unwrap(), query);
                            }
                            return FFIError::Ok;
                        }
                        Err(e) => {
                            debug!("error {:?}", e);
                            return e;
                        }
                    },
                    Err(e) => {
                        debug!("error {:?}", e);
                        return e;
                    }
                }
            }
        }
    }
    return FFIError::NullPointerPassed;
}

#[no_mangle]
pub extern "C" fn free_string(data: *mut i8) {
    debug!("freeing string");
    helpers::free_c_char(data);
}

#[no_mangle]
pub extern "C" fn free_string_ffi(incoming: Option<&StringFFI>, outgoing: Option<&mut StringFFI>) {
    if let Some(in_data) = incoming {
        if let Some(out_data) = outgoing {
            *out_data = *in_data;
            helpers::free_string(out_data.data);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn for_each_dft(
    ptr: InHandlePtr,
    node_visitor: CBFnNodeVisitor,
    start_node: *const c_char,
) -> FFIError {
    if let Some(handle) = ptr {
        if !start_node.is_null() {
            let c_str = unsafe {
                // assert!(!start_node.is_null());

                CStr::from_ptr(start_node)
            };
            let r_str = c_str.to_str().unwrap();
            debug!("start node name {}", r_str);

            // return Handle::from_ptr(ptr).for_each_dft(node_visitor, r_str.to_string());
            return handle.for_each_dft(node_visitor, r_str.to_string());
        } else {
            return FFIError::InvalidStringPassed;
        }
    }

    return FFIError::NullPointerPassed;
}

#[no_mangle]
pub unsafe extern "C" fn distance_to_other(
    ptr: InHandlePtr,
    node_name1: *const c_char,
    node_name2: *const c_char,
) -> f32 {
    if let Some(handle) = ptr {
        let node1 = handle.find_node(helpers::c_char_to_string(node_name1));
        let node2 = handle.find_node(helpers::c_char_to_string(node_name2));

        if let Ok(node1) = node1 {
            if let Ok(node2) = node2 {
                let distance = node1.distance_to_other(handle.data().unwrap(), node2);
                debug!("distance between selected {}", distance);
                return distance;
            } else {
                return -1f32;
            }
        } else {
            return -1f32;
        }
    }

    return -1f32;
}

#[no_mangle]
pub unsafe extern "C" fn test_cakes_rnn_query(
    ptr: InHandlePtr,
    search_radius: f32,
    node_visitor: CBFnNodeVisitor,
) -> FFIError {
    if let Some(handle) = ptr {
        let num_queries = 1;

        for j in 0..1000 {
            let queries = abd_clam::utils::helpers::gen_data_f32(num_queries, 10, 0., 1., j);
            let queries = queries.iter().collect::<Vec<_>>();
            for i in 0..num_queries {
                let (query, radius, _) = (&queries[i], search_radius, 10);
                handle.set_current_query(query);
                let rnn_results = handle.rnn_search(query, radius);
                match rnn_results {
                    Ok((confirmed, straddlers)) => {
                        if straddlers.len() < 5 || confirmed.len() < 5 {
                            continue;
                        }

                        for (cluster, dist) in &confirmed {
                            let mut baton = ClusterDataWrapper::from_cluster(cluster);
                            baton.data_mut().dist_to_query = *dist;
                            baton.data_mut().set_color(glam::Vec3 {
                                x: 0f32,
                                y: 1f32,
                                z: 0f32,
                            });
                            node_visitor(Some(baton.data()));

                            // baton.free_ids();
                        }

                        for (cluster, dist) in &straddlers {
                            let mut baton = ClusterDataWrapper::from_cluster(cluster);
                            baton.data_mut().dist_to_query = *dist;

                            baton.data_mut().set_color(glam::Vec3 {
                                x: 0f32,
                                y: 1f32,
                                z: 1f32,
                            });
                            node_visitor(Some(baton.data()));
                            // baton.free_ids();
                        }

                        return FFIError::Ok;
                    }
                    Err(_) => {
                        debug!("rnn failes");
                        // return e;
                    }
                }
            }
        }

        // let c_str = unsafe {
        //     if start_node.is_null() {
        //         return FFIError::NullPointerPassed;
        //     }

        //     CStr::from_ptr(start_node)
        // };
        // debug!("cakes quer ystart node {:?}", c_str);
        // match handle.find_node(String::from(c_str.to_str().unwrap())) {
        //     Ok(node) => {
        //         // node.

        //     }
        //     Err(e) => {
        //         return e;
        //     }
        // }
    }

    return FFIError::Ok;
}

// Option<& mut *mut Rc<RefCell<Handle>>>
// ptr: Option<&mut Rc<RefCell<Handle>>>
#[no_mangle]
pub unsafe extern "C" fn get_num_nodes(ptr: InHandlePtr) -> i32 {
    // Handle::from_ptr(ptr).get_num_nodes() + 1

    if let Some(handle) = ptr {
        return handle.get_num_nodes() + 1;
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn tree_height(ptr: InHandlePtr) -> i32 {
    // Handle::from_ptr(ptr).get_num_nodes() + 1

    if let Some(handle) = ptr {
        debug!("cardinality: {}", handle.tree_height() + 1);

        return handle.tree_height() + 1;
    }
    debug!("handle not created");

    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn cardinality(ptr: InHandlePtr) -> i32 {
    // Handle::from_ptr(ptr).get_num_nodes() + 1

    if let Some(handle) = ptr {
        debug!("cardinality: {}", handle.cardinality());
        return handle.cardinality();
    }
    debug!("handle not created");
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn radius(ptr: InHandlePtr) -> f64 {
    // Handle::from_ptr(ptr).get_num_nodes() + 1

    if let Some(handle) = ptr {
        debug!("radius: {}", handle.radius());
        return handle.radius();
    }
    debug!("handle not created");
    return 0.;
}

#[no_mangle]
pub unsafe extern "C" fn lfd(ptr: InHandlePtr) -> f64 {
    // Handle::from_ptr(ptr).get_num_nodes() + 1

    if let Some(handle) = ptr {
        debug!("rarg adius: {}", handle.lfd());
        return handle.lfd();
    }
    debug!("handle not created");
    return 0.;
}

#[no_mangle]
pub unsafe extern "C" fn arg_center(ptr: InHandlePtr) -> i32 {
    // Handle::from_ptr(ptr).get_num_nodes() + 1

    if let Some(handle) = ptr {
        debug!("rarg adius: {}", handle.arg_center());
        return handle.arg_center();
    }
    debug!("handle not created");
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn arg_radius(ptr: InHandlePtr) -> i32 {
    // Handle::from_ptr(ptr).get_num_nodes() + 1

    if let Some(handle) = ptr {
        debug!("rarg adius: {}", handle.arg_radius());
        return handle.arg_radius();
    }
    debug!("handle not created");
    return 0;
}
