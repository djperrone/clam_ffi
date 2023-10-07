use std::ffi::{c_char, CStr};

use crate::{
    debug,
    utils::{error::FFIError, helpers, types::InHandlePtr},
    CBFnNodeVisitor,
};

use super::{cluster_data::ClusterData, cluster_data_wrapper::ClusterDataWrapper};

pub unsafe fn for_each_dft_impl(
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

pub unsafe fn tree_height_impl(ptr: InHandlePtr) -> i32 {
    // Handle::from_ptr(ptr).get_num_nodes() + 1

    if let Some(handle) = ptr {
        debug!("cardinality: {}", handle.tree_height() + 1);

        return handle.tree_height() + 1;
    }
    debug!("handle not created");

    return 0;
}

pub unsafe fn get_cluster_data_impl(
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

pub unsafe fn color_by_dist_to_query_impl(
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

pub unsafe fn distance_to_other_impl(
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

pub unsafe fn test_cakes_rnn_query_impl(
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
                        }

                        return FFIError::Ok;
                    }
                    Err(_) => {
                        debug!("rnn failes");
                    }
                }
            }
        }
    }

    return FFIError::Ok;
}
