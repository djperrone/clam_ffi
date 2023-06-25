extern crate nalgebra as na;

use abd_clam::core::cluster::Cluster;
// use abd_clam::core::cluster_criteria::PartitionCriteria;
// use abd_clam::core::dataset::VecVec;
use ndarray::Data;
use std::cell::{RefCell, RefMut};
use std::ffi;
use std::mem::transmute;
use std::rc::Rc;

use abd_clam::cluster::PartitionCriteria;
use abd_clam::dataset::VecVec;
use abd_clam::search::cakes::CAKES;
// use abd_clam::utils::synthetic_data;

use crate::utils::error::FFIError;
use crate::utils::{anomaly_readers, distances, helpers};

use crate::{debug, InHandlePtr};

use super::node::NodeData;
use super::reingold_impl::{self};

pub type Clusterf32 = Cluster<f32, f32, VecVec<f32, f32>>;
type DataSet = VecVec<f32, f32>;
type Cakesf32 = CAKES<f32, f32, VecVec<f32, f32>>;

// either leaf node or
// depth at least 4
// lfd - btwn 0.5 - 2.5
// color clusters by radius or lfd
// draw clusters with lfd value

// noedges btwn parents and children
// want edges btwn two clustesr whose distbtwn centers <= sum of radius
// add radius and lfd to display info

pub struct Droppable {
    num: i32,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        debug!("Dropping drroppable");
    }
}

//tokio
// pub struct Handle2 {
//     root: Clusterf32,
//     dataset: & DataSet,
//     labels: & [u8],
// }

pub struct Handle {
    cakes: Option<Cakesf32>,
    // dataset: Option<DataSet>,
    labels: Option<Vec<u8>>,
    // droppable: Option<Droppable>,
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

    pub fn new(data_name: &str, cardinality: usize) -> Result<Self, FFIError> {
        let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);
        match Self::create_dataset(data_name) {
            Ok((dataset, labels)) => {
                return Ok(Handle {
                    cakes: Some(CAKES::new(dataset, Some(1)).build(&criteria)),
                    labels: Some(labels),
                });
            }
            Err(e) => Err(FFIError::HandleInitFailed),
        }
    }

    // pub fn to_ptr(self) -> *mut Handle {
    //     unsafe { transmute(Box::new(self)) }
    // }

    // pub fn from_ptr(ptr: *mut Handle) -> &mut Handle {
    //     unsafe { &mut *ptr }
    // }

    // pub unsafe fn from_smart_ptr(ptr: InHandlePtr) -> Result<RefMut<Handle>, FFIError> {
    //     // unsafe { &mut *ptr }

    //     if let Some(handle) = ptr {
    //         return Ok(handle);
    //     }

    //     return Err(FFIError::NullPointerPassed);
    // }
    pub fn default() -> Self {
        Handle {
            cakes: None,
            // dataset: None,
            labels: None,
            // droppable: Some(Droppable { num: 5 }),
        }
    }
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

    // pub fn build_abd_clam(
    //     &mut self,
    //     cardinality: usize,
    // ) -> Result<Rc<RefCell<Clusterf32>>, String> {
    //     let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);
    //     if let Some(dataset) = &self.dataset {
    //         return Ok(Rc::new(RefCell::new(
    //             Cluster::new_root(dataset)
    //                 .with_seed(1)
    //                 .partition(&criteria, true),
    //         )));
    //     } else {
    //         return Err("invalid dataset".to_string());
    //     }
    // }

    pub fn init_dataset(&mut self, data_name: &str) -> i32 {
        debug!("dataname in init_Dataset {}", data_name);
        match Self::create_dataset(data_name) {
            Ok((data, labels)) => {
                // self.dataset = Some(data);
                self.labels = Some(labels);
                return 1;
            }
            Err(e) => {
                debug!("{}", e);
                return 0;
            }
        }
    }

    pub unsafe fn for_each_dft(
        &self,
        node_visitor: crate::CBFnNodeVistor,
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

    fn for_each_dft_helper(root: &Clusterf32, node_visitor: crate::CBFnNodeVistor) {
        // if root.is_leaf() {
        //     let mut baton = NodeData::from_abd_clam(&root);

        //     node_visitor(Some(&baton));
        //     baton.free_ids();
        //     return;
        // }
        // if let Some([left, right]) = root.children() {
        //     let mut baton = NodeData::from_abd_clam(&root);

        //     node_visitor(Some(&baton));

        //     baton.free_ids();

        //     Self::for_each_dft_helper(left, node_visitor);
        //     Self::for_each_dft_helper(right, node_visitor);
        // }
    }

    pub fn get_num_nodes(&self) -> i32 {
        if let Some(cakes) = &self.cakes {
            cakes.tree().root().num_descendants() as i32
        } else {
            0
        }
    }
    // pub fn set_root(& mut self, root: Rc<RefCell<Clusterf32>>) {
    //     self.abd_clam_root = Some(root.clone());
    // }
    // pub fn get_dataset(&self) -> &DataSet {
    //     &self.cakes.unwrap().tree().data()
    // }

    // pub fn get_root(&self) -> &Clusterf32 {
    //     self.cakes.unwrap().tree().root()
    // }

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

    // pub fn find_node(&self, path: String) -> Result<NodeData, String> {
    //     if let Some(root) = self.abd_clam_root.clone() {
    //         let root = root.as_ref().borrow();
    //         // debug!(
    //         //     "searching for (node name in hex): {}",
    //         //     // unity_node.id.as_string()
    //         // );
    //         let mut path: String = helpers::hex_to_binary(path)
    //             .trim_start_matches('0')
    //             .chars()
    //             .rev()
    //             .collect();
    //         path.pop();

    //         let out = Self::find_node_helper(
    //             &root,
    //             // abd_clam_helpers:: unity_node.id.as_string().chars().rev().collect(),
    //             path,
    //         );

    //         return out;
    //     }
    //     debug!("root not built");
    //     return Err("root not built".to_string());
    // }

    // pub fn find_node_helper(root: &Clusterf32, mut path: String) -> Result<NodeData, String> {
    //     if path.len() == 0 {
    //         return Ok(NodeData::from_abd_clam(root));
    //     }
    //     let choice: char = path.pop().unwrap();
    //     if let Some([left, right]) = root.children() {
    //         if choice == '0' {
    //             return Self::find_node_helper(left, path);
    //         } else if choice == '1' {
    //             return Self::find_node_helper(right, path);
    //         } else {
    //             return Err("invalid character in node name".to_string());
    //         }
    //     } else {
    //         return Err("node not found - no children available".to_string());
    //     }
    // }

    pub fn create_reingold_layout(&self, node_visitor: crate::CBFnNodeVistor) -> FFIError {
        if let Some(cakes) = &self.cakes {
            if let Some(labels) = &self.labels {
                let layout_root = reingold_impl::Node::init_draw_tree(cakes.tree().root(), labels);

                let result = Self::reingoldify(layout_root, node_visitor);
                return result;
            } else {
                return FFIError::HandleInitFailed;
            }
        } else {
            return FFIError::HandleInitFailed;
        }
    }

    pub fn reingoldify(root: reingold_impl::Link, node_visitor: crate::CBFnNodeVistor) -> FFIError {
        if let Some(_) = root.clone() {
            Self::reingoldify_helper(root.clone(), node_visitor);

            return FFIError::Ok;
        }
        return FFIError::NullPointerPassed;
    }

    fn reingoldify_helper(root: reingold_impl::Link, node_visitor: crate::CBFnNodeVistor) -> () {
        if let Some(node) = root {
            let mut baton = NodeData::from_reingold_node(&node.as_ref().borrow());

            node_visitor(Some(&baton));
            baton.free_ids();

            Self::reingoldify_helper(node.as_ref().borrow().get_left_child(), node_visitor);
            Self::reingoldify_helper(node.as_ref().borrow().get_right_child(), node_visitor);
        }
    }
}
