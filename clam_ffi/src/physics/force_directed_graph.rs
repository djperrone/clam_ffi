// use super::node::NodeData;
use super::physics_node::PhysicsNode;
use super::spring::Spring;
use crate::ffi_impl::node::NodeData;
use crate::utils::error::FFIError;
use crate::{debug, CBFnNodeVisitor};
use std::collections::HashMap;

use std::sync::{Condvar, Mutex};

pub struct Status {
    pub data_ready: bool,
    // pub finished: bool,
}

impl Status {
    pub fn new() -> Self {
        Status {
            data_ready: false,
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
    unity_updater: CBFnNodeVisitor,
    max_iters: i32,
    // finished: bool,
}

impl ForceDirectedGraph {
    pub fn new(
        graph: HashMap<String, PhysicsNode>,
        edges: Vec<Spring>,
        scalar: f32,
        max_iters: i32,
        unity_updater: CBFnNodeVisitor,
    ) -> Self {
        let max_edge_len = Self::calc_max_edge_len(&edges);

        ForceDirectedGraph {
            graph: Mutex::new((Status::new(), graph)),
            edges: edges,
            max_edge_len: max_edge_len,
            scalar: scalar,
            // data_ready: Mutex::new(false),
            cond_var: Condvar::new(),
            unity_updater: unity_updater,
            max_iters: max_iters,
            // finished: false,
        }
    }

    fn compute_next_frame(&self) {
        let mutex_result = self
            .cond_var
            .wait_while(self.graph.lock().unwrap(), |(status, _)| {
                status.data_ready == true
            });

        match mutex_result {
            Ok(mut g) => {
                // let data_ready = &mut g.0;
                // let graph = &mut g.1;
                for spring in self.edges.iter() {
                    spring.move_nodes(&mut g.1, self.max_edge_len, self.scalar);
                }

                g.0.data_ready = true;
                debug!("set data reasdy true");
            }
            Err(e) => {
                debug!("graph mutex error? {}", e);
            }
        }

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

    fn try_update_unity(&self) -> FFIError {
        match self.graph.try_lock() {
            Ok(mut g) => {
                // Ok(mut graph) => {
                for (key, value) in &mut g.1 {
                    value.update_position();
                    let mut ffi_data = NodeData::new(key.clone());
                    ffi_data.set_position(value.get_position());
                    (self.unity_updater)(Some(&ffi_data));
                    ffi_data.free_ids();
                }

                // if g.0.finished == true {
                //     debug!("finished physics sim");
                //     return FFIError::PhysicsFinished;
                // }

                // if self.

                g.0.data_ready = false;
                debug!("set data ready false");
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
    debug!("num iters: {}", force_directed_graph.max_iters);
    for i in 0..force_directed_graph.max_iters {
        println!("p: {}", i);
        force_directed_graph.compute_next_frame();
    }
}

pub fn try_update_unity(force_directed_graph: &ForceDirectedGraph) -> FFIError {
    return force_directed_graph.try_update_unity();
}
