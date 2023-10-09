// use super::node::NodeData;
use super::physics_node::PhysicsNode;
use super::spring::Spring;
// use crate::ffi_impl::cluster_data::ClusterData;
use crate::ffi_impl::cluster_data_wrapper::ClusterDataWrapper;
use crate::utils::error::FFIError;
use crate::{debug, CBFnNodeVisitor};
use std::collections::HashMap;

use std::sync::{Condvar, Mutex};

pub struct Status {
    pub data_ready: bool,
    pub force_shutdown: bool,
    // pub finished: bool,
}

impl Status {
    pub fn new() -> Self {
        Status {
            data_ready: false,
            force_shutdown: false,
            // probably not needed - thread has .isfinished...
            // finished: false,
        }
    }
}

pub struct ForceDirectedGraph {
    graph: Mutex<(Status, HashMap<String, PhysicsNode>)>,
    edges: Vec<Spring>,
    max_edge_len: f32,
    scalar: f32,
    // data_ready: Mutex<bool>,
    cond_var: Condvar,
    // unity_updater: CBFnNodeVisitor,
    max_iters: i32,
    // finished: bool,
}

impl ForceDirectedGraph {
    pub fn new(
        graph: HashMap<String, PhysicsNode>,
        edges: Vec<Spring>,
        scalar: f32,
        max_iters: i32,
        // unity_updater: CBFnNodeVisitor,
    ) -> Self {
        let max_edge_len = Self::calc_max_edge_len(&edges);

        ForceDirectedGraph {
            graph: Mutex::new((Status::new(), graph)),
            edges: edges,
            max_edge_len: max_edge_len,
            scalar: scalar,
            // data_ready: Mutex::new(false),
            cond_var: Condvar::new(),
            // unity_updater: unity_updater,
            max_iters: max_iters,
            // finished: false,
        }
    }

    fn compute_next_frame(&self) -> bool {
        let mutex_result = self
            .cond_var
            .wait_while(self.graph.lock().unwrap(), |(status, _)| {
                status.data_ready == true && status.force_shutdown == false
            });
        let i = 6;

        // mutex_result.unwrap().0.force_shutdown;

        match mutex_result {
            Ok(mut g) => {
                if g.0.force_shutdown == true {
                    g.0.data_ready = false;
                    return false;
                } else {
                    // let graph = &mut g.1;
                    for spring in self.edges.iter() {
                        spring.move_nodes(&mut g.1, self.max_edge_len, self.scalar);
                    }

                    g.0.data_ready = true;
                }
                // let data_ready = &mut g.0;
            }
            Err(e) => {
                debug!("graph mutex error? {}", e);
            }
        }

        return true;

        // match self.graph.lock() {
        //     Ok(mut g) => {
        //         for spring in self.edges.iter() {
        //             spring.move_nodes(&mut g, self.max_edge_len, self.scalar);
        //         }
        //         *data_ready = true;
        //     }
        //     Err(e) => {
        //         debug!("graph mutex error? {}", e);
        //     }
        // }
    }

    unsafe fn force_shutdown(&self) -> FFIError {
        debug!("trying to end sim early - force shutdown lock");

        match self.graph.lock() {
            Ok(mut g) => {
                g.0.force_shutdown = true;

                // Ok(mut graph) => {
                // for (key, value) in &mut g.1 {
                //     // debug!("updating node {}", key);
                //     value.update_position();
                //     let baton_data =
                //         ClusterDataWrapper::from_physics(key.clone(), value.get_position());

                //     updater(Some(baton_data.data()));

                // }

                // debug!("updated all nodes");

                // g.0.data_ready = false;
                // debug!("set data ready false");
                self.cond_var.notify_all();
                // debug!("notified thread");

                return FFIError::PhysicsRunning;
            }
            Err(e) => {
                debug!("Data not ready...try again later {}", e);
                return FFIError::PhysicsNotReady;
            }
        }
    }

    unsafe fn try_update_unity(&self, updater: CBFnNodeVisitor) -> FFIError {
        match self.graph.try_lock() {
            Ok(mut g) => {
                // Ok(mut graph) => {
                for (key, value) in &mut g.1 {
                    // debug!("updating node {}", key);
                    value.update_position();
                    let baton_data =
                        ClusterDataWrapper::from_physics(key.clone(), value.get_position());
                    // let mut ffi_data =
                    //     ClusterData::from_physics(key.clone(), value.get_position());
                    // let mut ffi_data = ClusterData::new(key.clone());
                    // debug!("updating node count 2");

                    // ffi_data.set_position(value.get_position());
                    // debug!("updating node count 3");
                    //assert ffi data is valid here
                    // if ffi_data.get_ffi_id().data.is_null() {
                    //     debug!("id is null?");
                    //     panic!();
                    // }
                    updater(Some(baton_data.data()));
                    // (self.unity_updater)(Some(&ffi_data));
                    // debug!("updating node count 4");

                    // ffi_data.free_ids();
                    // debug!("updated node {}", key);
                }

                // if g.0.finished == true {
                //     debug!("finished physics sim");
                //     return FFIError::PhysicsFinished;
                // }

                // if self.

                g.0.data_ready = false;
                self.cond_var.notify_one();

                return FFIError::PhysicsRunning;
            }
            Err(e) => {
                debug!("Data not ready...try again later {}", e);
                return FFIError::PhysicsNotReady;
            }
        }
        // let mut data_ready = self
        // return FFIError::PhysicsRunning;
        //     .cond_var
        //     .wait_while(self.data_ready.lock().unwrap(), |ready| *ready == true)
        //     .unwrap();

        // match self.graph.lock() {
        //     Ok(mut g) => {
        //         for spring in self.edges.iter() {
        //             spring.move_nodes(&mut g, self.max_edge_len, self.scalar);
        //         }
        //         *data_ready = true;
        //     }
        //     Err(e) => {
        //         debug!("graph mutex error? {}", e);
        //     }
        // }
    }

    // fn run(graph: HashMap<String, PhysicsNode>, edges: Vec<Spring>, scalar: f32) {
    //     let buffer = Arc::new(ForceDirectedGraph::new(graph, edges, scalar));

    //     let b = buffer.clone();
    //     let p = thread::spawn(move || {
    //         produce_computations(&b);
    //     });

    //     // thread::spawn(move || {
    //     //     let mut data_ready = &self.data_ready.lock().unwrap();
    //     //     if data_ready {
    //     //         //update
    //     //         //sleep
    //     //     } else {
    //     //     }
    //     //     // We notify the condvar that the value has changed.
    //     //     // cvar.notify_one();
    //     // });
    // }

    fn calc_max_edge_len(edges: &Vec<Spring>) -> f32 {
        let max_edge_len: f32 = edges
            .iter()
            .reduce(|cur_max: &Spring, val: &Spring| {
                if cur_max.nat_len() > val.nat_len() {
                    cur_max
                } else {
                    val
                }
            })
            .unwrap()
            .nat_len();

        max_edge_len
    }
}

pub fn produce_computations(force_directed_graph: &ForceDirectedGraph) {
    for i in 0..force_directed_graph.max_iters {
        println!("p: {}", i);

        // returns false if being forced to terminate mid - simulation
        if force_directed_graph.compute_next_frame() == false {
            return;
        };
    }
}

pub unsafe fn try_update_unity(
    force_directed_graph: &ForceDirectedGraph,
    updater: CBFnNodeVisitor,
) -> FFIError {
    return force_directed_graph.try_update_unity(updater);
}

pub unsafe fn force_shutdown(force_directed_graph: &ForceDirectedGraph) -> FFIError {
    return force_directed_graph.force_shutdown();
}
