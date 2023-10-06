extern crate nalgebra as na;

use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use abd_clam::cluster::PartitionCriteria;
use abd_clam::dataset::VecVec;
use abd_clam::search::cakes::CAKES;
use glam::Vec3;

use crate::physics::force_directed_graph::ForceDirectedGraph;
use crate::physics::{self, spring};
use crate::tree_layout::reingold_tilford;
use crate::utils::error::FFIError;
use crate::utils::types::{Cakesf32, Clusterf32, DataSet};
use crate::utils::{anomaly_readers, distances, helpers};

use crate::{debug, CBFnNodeVisitor};

use super::cluster_data::ClusterData;
use super::cluster_data_wrapper::ClusterDataWrapper;
// use super::reingold_impl::{self};
use crate::physics::physics_node::PhysicsNode;
use spring::Spring;
// use crate::physics::ForceDirectedGraph;

// use ForceDirectedGraph;

// either leaf node or
// depth at least 4
// lfd - btwn 0.5 - 2.5
// color clusters by radius or lfd
// draw clusters with lfd value

// noedges btwn parents and children
// want edges btwn two clustesr whose distbtwn centers <= sum of radius
// add radius and lfd to display info

//tokio
// pub struct Handle2 {
//     root: Clusterf32,
//     dataset: & DataSet,
//     labels: & [u8],
// }

pub struct Handle {
    cakes: Option<Cakesf32>,
    labels: Option<Vec<u8>>,
    graph: Option<HashMap<String, PhysicsNode>>,
    edges: Option<Vec<Spring>>,
    current_query: Option<Vec<f32>>,
    longest_edge: Option<f32>,

    force_directed_graph: Option<(JoinHandle<()>, Arc<ForceDirectedGraph>)>,
}

impl Drop for Handle {
    fn drop(&mut self) {
        debug!("DroppingHandle");
    }
}
impl Handle {
    pub fn shutdown(&mut self) {
        self.cakes = None;
        // self.dataset = None;
        self.labels = None;
    }

    pub fn data(&self) -> Option<&DataSet> {
        if let Some(c) = &self.cakes {
            return Some(c.data());
        } else {
            return None;
        }
    }
    pub fn root(&self) -> Option<&Clusterf32> {
        if let Some(c) = &self.cakes {
            return Some(c.tree().root());
        } else {
            return None;
        }
    }

    pub fn new(data_name: &str, cardinality: usize) -> Result<Self, FFIError> {
        if data_name == "test" {
            let seed = 42;
            let data = abd_clam::utils::helpers::gen_data_f32(2000, 2, 0., 1., seed);
            let dataset = VecVec::new(data, distances::euclidean_sq, "1k-10".to_string(), false);
            let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);

            let cakes = CAKES::new(dataset, Some(seed)).build(&criteria);
            // *out_handle = Box::into_raw(Box::new(cakes));
            let handle = Handle {
                cakes: Some(cakes),
                labels: None,
                graph: None,
                edges: None,
                current_query: None,
                longest_edge: None,
                force_directed_graph: None,
            };

            return Ok(handle);
        }

        if data_name == "rand" {
            let seed = 42;
            let data = abd_clam::utils::helpers::gen_data_f32(100_00, 10, 0., 1., seed);
            let dataset = VecVec::new(data, distances::euclidean_sq, "100k-10".to_string(), false);
            let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);

            let cakes = CAKES::new(dataset, Some(seed)).build(&criteria);
            // *out_handle = Box::into_raw(Box::new(cakes));
            let handle = Handle {
                cakes: Some(cakes),
                labels: None,
                graph: None,
                edges: None,
                current_query: None,
                longest_edge: None,
                force_directed_graph: None,
            };

            return Ok(handle);
        }

        let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);
        match Self::create_dataset(data_name) {
            Ok((dataset, labels)) => {
                return Ok(Handle {
                    cakes: Some(CAKES::new(dataset, Some(1)).build(&criteria)),
                    labels: Some(labels),
                    graph: None,
                    edges: None,
                    current_query: None,
                    longest_edge: None,
                    force_directed_graph: None,
                });
            }
            Err(_) => Err(FFIError::HandleInitFailed),
        }
    }

    // pub fn to_ptr(self) -> *mut Handle {
    //     unsafe { transmute(Box::new(self)) }
    // }

    // pub unsafe fn from_ptr(ptr: *mut Handle) -> &mut Handle {
    //     &mut *ptr
    // }

    // pub fn default() -> Self {
    //     Handle {
    //         cakes: None,
    //         // dataset: None,
    //         labels: None,
    //         graph: None,
    //         edges: None,
    //         current_query: None,
    //         longest_edge : None
    //         // droppable: Some(Droppable { num: 5 }),
    //     }
    // }
    fn create_dataset(data_name: &str) -> Result<(DataSet, Vec<u8>), String> {
        match anomaly_readers::read_anomaly_data(data_name, false) {
            Ok((first_data, labels)) => {
                let dataset = VecVec::new(
                    first_data,
                    distances::euclidean_sq,
                    data_name.to_string(),
                    false,
                );

                Ok((dataset, labels))
            }
            Err(e) => Err(e),
        }
    }

    // pub unsafe fn build_force_directed_graph(
    //     &mut self,
    //     cluster_data_arr: &[ClusterData],
    //     scalar: f32,
    //     max_iters: i32,
    //     edge_detector_cb: CBFnNodeVisitor,
    //     // physics_update_cb: CBFnNodeVisitor,
    // ) -> FFIError {
    //     let mut clusters: Vec<&Clusterf32> = Vec::new();

    //     for c in cluster_data_arr {
    //         if let Ok(cluster) = self.find_node(c.id.as_string().unwrap()) {
    //             clusters.push(cluster);
    //         }
    //     }

    //     let springs: Vec<Spring> =
    //         Self::create_springs(&self.detect_edges(&clusters, edge_detector_cb));
    //     let graph = self.build_graph(&cluster_data_arr);
    //     if graph.len() == 0 || springs.len() == 0 {
    //         return FFIError::GraphBuildFailed;
    //     }

    //     let force_directed_graph = Arc::new(ForceDirectedGraph::new(
    //         graph, springs, scalar, max_iters,
    //         // physics_update_cb,
    //     ));

    //     let b = force_directed_graph.clone();
    //     let p = thread::spawn(move || {
    //         physics::force_directed_graph::produce_computations(&b);
    //     });
    //     self.force_directed_graph = Some((p, force_directed_graph.clone()));

    //     return FFIError::Ok;
    // }

    pub unsafe fn physics_update_async(&mut self, updater: CBFnNodeVisitor) -> FFIError {
        // let mut finished = false;
        if let Some(force_directed_graph) = &self.force_directed_graph {
            debug!("fdg exists");

            let is_finished = force_directed_graph.0.is_finished();

            if is_finished {
                let _ = self.force_directed_graph.take().unwrap().0.join();
                self.force_directed_graph = None;
                debug!("shutting down physics");
                return FFIError::PhysicsFinished;
            } else {
                debug!("try to update unity");

                return physics::force_directed_graph::try_update_unity(
                    &force_directed_graph.1,
                    updater,
                );
            }
            // let update_result =
            //     physics::force_directed_graph::try_update_unity(&force_directed_graph.1);
        }

        return FFIError::PhysicsAlreadyShutdown;
    }

    // pub unsafe fn init_force_directed_sim(
    //     &mut self,
    //     cluster_data_arr: &[ClusterData],
    //     node_visitor: CBFnNodeVisitor,
    // ) -> FFIError {
    //     let mut clusters: Vec<&Clusterf32> = Vec::new();

    //     for c in cluster_data_arr {
    //         if let Ok(cluster) = self.find_node(c.id.as_string().unwrap()) {
    //             clusters.push(cluster);
    //         }
    //     }

    //     let springs: Vec<Spring> =
    //         Self::create_springs(&self.detect_edges(&clusters, node_visitor));
    //     let graph = self.build_graph(&cluster_data_arr);
    //     if graph.len() == 0 || springs.len() == 0 {
    //         return FFIError::GraphBuildFailed;
    //     }

    //     self.graph = Some(graph);
    //     self.edges = Some(springs);

    //     let longest_edge: f32 = self
    //         .edges
    //         .as_ref()
    //         .unwrap()
    //         .iter()
    //         .reduce(|cur_max: &Spring, val: &Spring| {
    //             if cur_max.nat_len() > val.nat_len() {
    //                 cur_max
    //             } else {
    //                 val
    //             }
    //         })
    //         .unwrap()
    //         .nat_len();

    //     if longest_edge == 0. {
    //         debug!("error: longest edge is 0");
    //         return FFIError::DivisionByZero;
    //     }
    //     self.longest_edge = Some(longest_edge);

    //     return FFIError::Ok;
    // }

    // pub fn apply_forces(&mut self, node_visitor: crate::CBFnNodeVisitor, scalar: f32) -> FFIError {
    //     if let Some(graph) = &mut self.graph {
    //         if let Some(springs) = &self.edges {
    //             for spring in springs.iter() {
    //                 spring.move_nodes(graph, self.longest_edge.unwrap(), scalar);
    //             }

    //             //update position on canvas of all nodes (after all spring forces applied, each node will have its final acceleration for this frame)
    //             for (key, value) in graph {
    //                 value.update_position();
    //                 let mut ffi_data = ClusterData::new(key.clone());
    //                 ffi_data.set_position(value.get_position());
    //                 node_visitor(Some(&ffi_data));
    //                 ffi_data.free_ids();
    //             }

    //             return FFIError::Ok;
    //         }
    //     }
    //     return FFIError::NullPointerPassed;
    // }
    // pub unsafe fn build_graph(
    //     &self,
    //     // clusters: &'a Vec<&'a Clusterf32>,
    //     cluster_data_arr: &[ClusterData],
    // ) -> HashMap<String, PhysicsNode> {
    //     let mut graph: HashMap<String, PhysicsNode> = HashMap::new();

    //     for c in cluster_data_arr {
    //         graph.insert(
    //             c.id.as_string().unwrap(),
    //             PhysicsNode::new(&c, self.find_node(c.id.as_string().unwrap()).unwrap()),
    //         );
    //     }

    //     return graph;
    // }

    // pub unsafe extern "C" fn second_build_graph(
    //     &mut self,
    //     arr_ptr: *mut NodeData,
    //     len: i32,
    //     scalar: f32,
    //     max_iters: i32,
    //     edge_detect_cb: CBFnNodeVisitor,
    //     physics_update_cb: CBFnNodeVisitor,
    // ) -> FFIError {

    // }

    pub fn set_graph(&mut self, graph: (JoinHandle<()>, Arc<ForceDirectedGraph>)) {
        self.force_directed_graph = Some(graph);
    }

    //creates spring for each edge in graph
    fn create_springs(edges_data: &Vec<(String, String, f32)>) -> Vec<Spring> {
        let spring_multiplier = 5.;

        let mut return_vec: Vec<Spring> = Vec::new();

        for data in edges_data {
            //resting length scaled by spring_multiplier
            // edge_lenght = data.2
            let new_spring =
                Spring::new(data.2 * spring_multiplier, data.0.clone(), data.1.clone());
            return_vec.push(new_spring);
        }

        return_vec
    }

    pub fn detect_edges(
        &self,
        clusters: &Vec<&Clusterf32>,
        node_visitor: crate::CBFnNodeVisitor,
    ) -> Vec<(String, String, f32)> {
        let mut edges: Vec<(String, String, f32)> = Vec::new();

        for i in 0..clusters.len() {
            for j in (i + 1)..clusters.len() {
                let distance = clusters[i].distance_to_other(self.data().unwrap(), clusters[j]);
                if distance <= clusters[i].radius() + clusters[j].radius() {
                    edges.push((clusters[i].name(), clusters[j].name(), distance));

                    let mut baton_data = ClusterDataWrapper::from_cluster(clusters[i]);
                    baton_data.data_mut().set_left_id(clusters[j].name());
                    node_visitor(Some(baton_data.data()));
                    // data.free_ids();
                }
            }
        }

        return edges;
    }

    pub unsafe fn color_by_dist_to_query(
        &self,
        id_arr: &[String],
        node_visitor: CBFnNodeVisitor,
    ) -> FFIError {
        for id in id_arr {
            match self.find_node(id.clone()) {
                Ok(cluster) => {
                    if let Some(query) = &self.current_query {
                        let mut baton_data = ClusterDataWrapper::from_cluster(cluster);

                        baton_data.data_mut().dist_to_query =
                            cluster.distance_to_instance(self.data().unwrap(), query);

                        node_visitor(Some(baton_data.data()));

                        // baton_data.free_ids();
                    } else {
                        return FFIError::QueryIsNull;
                    }
                }
                Err(e) => {
                    return e;
                }
            }
        }
        return FFIError::Ok;
    }

    pub unsafe fn for_each_dft(
        &self,
        node_visitor: crate::CBFnNodeVisitor,
        start_node: String,
    ) -> FFIError {
        if let Some(cakes) = &self.cakes {
            if start_node == "root" {
                let node = cakes.tree().root();
                Self::for_each_dft_helper(&node, node_visitor);
                return FFIError::Ok;
            } else {
                match Self::find_node(&self, start_node) {
                    Ok(root) => {
                        Self::for_each_dft_helper(root, node_visitor);
                        return FFIError::Ok;
                    }
                    Err(e) => {
                        debug!("{:?}", e);
                        return FFIError::InvalidStringPassed;
                    }
                }
            }
        } else {
            return FFIError::NullPointerPassed;
        }
    }

    fn for_each_dft_helper(root: &Clusterf32, node_visitor: crate::CBFnNodeVisitor) {
        if root.is_leaf() {
            let baton = ClusterDataWrapper::from_cluster(&root);

            node_visitor(Some(baton.data()));
            // baton.free_ids();
            return;
        }
        if let Some([left, right]) = root.children() {
            let baton = ClusterDataWrapper::from_cluster(&root);

            node_visitor(Some(&baton.data()));
            // baton.free_ids();

            Self::for_each_dft_helper(left, node_visitor);
            Self::for_each_dft_helper(right, node_visitor);
        }
    }

    pub fn shutdown_physics(&mut self) -> FFIError {
        let should_shutdown = { self.graph.is_some() && self.edges.is_some() };

        if should_shutdown {
            self.graph = None;
            self.edges = None;
            return FFIError::Ok;
        } else {
            return FFIError::PhysicsAlreadyShutdown;
        }
    }

    pub fn set_current_query(&mut self, data: &Vec<f32>) {
        self.current_query = Some(data.clone());
    }

    pub fn get_current_query(&self) -> &Option<Vec<f32>> {
        &self.current_query
    }

    pub fn rnn_search(
        &self,
        query: &Vec<f32>,
        radius: f32,
    ) -> Result<(Vec<(&Clusterf32, f32)>, Vec<(&Clusterf32, f32)>), FFIError> {
        if let Some(cakes) = &self.cakes {
            // temporary fix later
            // self.current_query = Some(query.clone());
            return Ok(cakes.rnn_search_candidates(query, radius));
        }
        return Err(FFIError::NullPointerPassed);
    }

    pub fn get_num_nodes(&self) -> i32 {
        if let Some(cakes) = &self.cakes {
            cakes.tree().root().num_descendants() as i32
        } else {
            0
        }
    }

    pub fn tree_height(&self) -> i32 {
        if let Some(cakes) = &self.cakes {
            cakes.tree().root().max_leaf_depth() as i32
        } else {
            0
        }
    }

    pub fn cardinality(&self) -> i32 {
        if let Some(cakes) = &self.cakes {
            cakes.tree().root().cardinality() as i32
        } else {
            0
        }
    }

    pub fn radius(&self) -> f64 {
        if let Some(cakes) = &self.cakes {
            cakes.tree().root().radius() as f64
        } else {
            0.
        }
    }

    pub fn lfd(&self) -> f64 {
        if let Some(cakes) = &self.cakes {
            cakes.tree().root().lfd()
        } else {
            0.
        }
    }

    pub fn arg_center(&self) -> i32 {
        if let Some(cakes) = &self.cakes {
            cakes.tree().root().arg_center() as i32
        } else {
            0
        }
    }
    pub fn arg_radius(&self) -> i32 {
        if let Some(cakes) = &self.cakes {
            cakes.tree().root().arg_radius() as i32
        } else {
            0
        }
    }

    // why isnt string taken by reference?
    pub unsafe fn find_node(&self, path: String) -> Result<&Clusterf32, FFIError> {
        if let Some(cakes) = &self.cakes {
            let mut path: String = helpers::hex_to_binary(path)
                .trim_start_matches('0')
                .chars()
                .rev()
                .collect();
            path.pop();

            return Self::find_node_helper(cakes.tree().root(), path);
        }
        debug!("root not built");
        return Err(FFIError::HandleInitFailed);
    }

    pub fn find_node_helper(root: &Clusterf32, mut path: String) -> Result<&Clusterf32, FFIError> {
        if path.len() == 0 {
            return Ok(&root);
        }
        let choice: char = path.pop().unwrap();
        if let Some([left, right]) = root.children() {
            if choice == '0' {
                return Self::find_node_helper(left, path);
            } else if choice == '1' {
                return Self::find_node_helper(right, path);
            } else {
                return Err(FFIError::InvalidStringPassed);
            }
        } else {
            return Err(FFIError::InvalidStringPassed);
        }
    }

    pub fn create_reingold_layout(&self, node_visitor: crate::CBFnNodeVisitor) -> FFIError {
        if let Some(cakes) = &self.cakes {
            return reingold_tilford::run(
                cakes.tree().root(),
                &self.labels,
                &self.data(),
                node_visitor,
            );
        } else {
            return FFIError::HandleInitFailed;
        }
    }

    pub unsafe fn create_reingold_layout_offset_from(
        &self,
        root: &ClusterData,
        node_visitor: crate::CBFnNodeVisitor,
    ) -> FFIError {
        if let Some(_) = &self.cakes {
            if let Ok(clam_root) = self.find_node(root.get_id()) {
                return reingold_tilford::run_offset(
                    &root.pos,
                    clam_root,
                    &self.labels,
                    &self.data(),
                    node_visitor,
                );
            } else {
                return FFIError::NullPointerPassed;
            }
        } else {
            return FFIError::HandleInitFailed;
        }
    }
}
