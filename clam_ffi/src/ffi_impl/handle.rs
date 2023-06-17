extern crate nalgebra as na;

use clam::core::cluster::Cluster;
use clam::core::cluster_criteria::PartitionCriteria;
use clam::core::dataset::VecVec;
use std::cell::RefCell;
use std::mem::transmute;
use std::rc::Rc;

use crate::ffi_impl::clam_helpers;
use crate::utils::{anomaly_readers, debug, distances, helpers};

use crate::debug;

use super::node::{NodeData2, NodeFFI};
use super::reingold_impl::{self};
pub type Clusterf32<'a> = Cluster<'a, f32, f32, VecVec<f32, f32>>;
type DataSet<'a> = VecVec<f32, f32>;

pub struct Handle<'a> {
    clam_root: Option<Rc<RefCell<Clusterf32<'a>>>>,
    dataset: Option<DataSet<'a>>,
    labels: Option<Vec<u8>>,
}

impl<'a> Handle<'a> {
    pub fn to_ptr(self) -> *mut Handle<'a> {
        unsafe { transmute(Box::new(self)) }
    }

    pub fn from_ptr(ptr: *mut Handle) -> &'a mut Handle {
        unsafe { &mut *ptr }
    }
    pub fn default() -> Self {
        Handle {
            clam_root: None,
            dataset: None,
            labels: None,
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

    pub fn build_clam(&mut self, cardinality: usize) -> Result<Rc<RefCell<Clusterf32>>, String> {
        let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);
        if let Some(dataset) = &self.dataset {
            return Ok(Rc::new(RefCell::new(
                Cluster::new_root(dataset)
                    .partition(&criteria, true)
                    .with_seed(1),
            )));
        } else {
            return Err("invalid dataset".to_string());
        }
    }

    pub fn init_dataset(&mut self, data_name: &str) -> i32 {
        debug!("dataname in init_Dataset {}", data_name);
        match Self::create_dataset(data_name) {
            Ok((data, labels)) => {
                self.dataset = Some(data);
                self.labels = Some(labels);
                return 1;
            }
            Err(e) => {
                debug!("{}", e);
                return 0;
            }
        }
    }

    pub fn traverse_tree_df(&mut self, node_visitor: crate::CBFnNodeVistor) -> i32 {
        if let Some(root) = &self.clam_root {
            let node = root.as_ref().borrow();
            Self::traverse_tree_df_helper(&node, node_visitor);
            return 1;
        }
        return 0;
    }

    fn traverse_tree_df_helper(root: &Clusterf32, node_visitor: crate::CBFnNodeVistor) {
        if root.is_leaf() {
            let baton = NodeFFI::from_clam(&root).to_ptr();

            node_visitor(baton);
            return;
        }
        if let Some([left, right]) = root.children() {
            let baton = NodeFFI::from_clam(&root).to_ptr();

            node_visitor(baton);

            unsafe {
                Box::from_raw(baton).free_ids();
            }

            Self::traverse_tree_df_helper(left, node_visitor);
            Self::traverse_tree_df_helper(right, node_visitor);
        }
    }

    pub fn traverse_tree_df2(&mut self, node_visitor: crate::CBFnNodeVistor2) -> i32 {
        if let Some(root) = &self.clam_root {
            let node = root.as_ref().borrow();
            Self::traverse_tree_df_helper2(&node, node_visitor);
            return 1;
        }
        return 0;
    }

    fn traverse_tree_df_helper2(root: &Clusterf32, node_visitor: crate::CBFnNodeVistor2) {
        if root.is_leaf() {
            let mut baton = NodeData2::from_clam(&root);

            node_visitor(Some(&baton));
            baton.free_ids();
            return;
        }
        if let Some([left, right]) = root.children() {
            let mut baton = NodeData2::from_clam(&root);

            node_visitor(Some(&baton));

            // unsafe {
            //     Box::from_raw(baton).free_ids();
            // }
            baton.free_ids();

            Self::traverse_tree_df_helper2(left, node_visitor);
            Self::traverse_tree_df_helper2(right, node_visitor);
        }
    }

    pub fn get_num_nodes(&self) -> i32 {
        if let Some(root) = &self.clam_root {
            root.as_ref().borrow().num_descendants() as i32
        } else {
            0
        }
    }
    pub fn set_root(&'a mut self, root: Rc<RefCell<Clusterf32<'a>>>) {
        self.clam_root = Some(root.clone());
    }
    pub fn get_dataset(&self) -> &DataSet {
        &self.dataset.as_ref().unwrap()
    }

    pub fn get_root(&self) -> &Option<Rc<RefCell<Clusterf32<'a>>>> {
        &self.clam_root
    }

    pub fn get_node_data_helper(root: &Clusterf32, mut name: String) -> Result<NodeFFI, String> {
        if name.len() == 0 {
            return Ok(NodeFFI::from_clam(root));
        }
        debug!("{:?}", name);
        let choice = name.pop();
        if let Some([left, right]) = root.children() {
            if choice.unwrap() == '0' {
                return Self::get_node_data_helper(left, name);
            } else if choice.unwrap() == '1' {
                return Self::get_node_data_helper(right, name);
            }
        } else {
            return Err("node not found - no children available".to_string());
        }

        return Err("node not found".to_string());
    }
    pub fn get_node_data(&self, name: String) -> Result<NodeFFI, String> {
        if let Some(node) = self.clam_root.clone() {
            let node = node.as_ref().borrow();
            return Self::get_node_data_helper(&node, name);
        }
        debug!("root not built");
        return Err("root not built".to_string());
    }

    pub fn get_node_data2(&self, path: String) -> Result<NodeData2, String> {
        if let Some(root) = self.clam_root.clone() {
            let root = root.as_ref().borrow();
            // debug!(
            //     "searching for (node name in hex): {}",
            //     // unity_node.id.as_string()
            // );
            debug!("path here {}", path);
            let mut path: String = helpers::hex_to_binary(path)
                .trim_start_matches('0')
                .chars()
                .rev()
                .collect();
            path.pop();

            debug!("path binary {}", path);
            let out = Self::get_node_data_helper2(
                &root,
                // clam_helpers:: unity_node.id.as_string().chars().rev().collect(),
                path,
            );

            return out;
        }
        debug!("root not built");
        return Err("root not built".to_string());
    }

    pub fn get_node_data_helper2(root: &Clusterf32, mut path: String) -> Result<NodeData2, String> {
        if path.len() == 0 {
            // unity_node.set_from_clam(root);
            // unity_node.cardinality = root.cardinality() as i32;
            // unity_node.depth = root.depth() as i32;
            // unity_node.arg_center = root.arg_center() as i32;
            // unity_node.arg_radius = root.arg_radius() as i32;
            return Ok(NodeData2::from_clam(root));
        }
        // debug!("{:?}", name);
        let choice: char = path.pop().unwrap();
        if let Some([left, right]) = root.children() {
            if choice == '0' {
                return Self::get_node_data_helper2(left, path);
            } else if choice == '1' {
                return Self::get_node_data_helper2(right, path);
            }
        } else {
            return Err("node not found - no children available".to_string());
        }

        return Err("node not found".to_string());
    }

    pub fn create_reingold_layout(&mut self, node_visitor: crate::CBFnNodeVistor) -> i32 {
        if let Some(root) = &self.clam_root {
            if let Some(labels) = &self.labels {
                let layout_root =
                    reingold_impl::Node::init_draw_tree(&root.as_ref().borrow(), labels);

                let result = Self::reingoldify(layout_root, node_visitor);
                return result;
            } else {
                return -3;
            }
        } else {
            return -4;
        }
    }

    pub fn reingoldify(root: reingold_impl::Link, node_visitor: crate::CBFnNodeVistor) -> i32 {
        if let Some(_) = root.clone() {
            Self::reingoldify_helper(root.clone(), node_visitor);

            return 1;
        }
        return -1;
    }

    fn reingoldify_helper(root: reingold_impl::Link, node_visitor: crate::CBFnNodeVistor) -> () {
        if let Some(node) = root {
            let baton = NodeFFI::from_reingold_node(&node.as_ref().borrow()).to_ptr();

            node_visitor(baton);
            unsafe {
                NodeFFI::from_ptr(baton).free_ids();
            }

            Self::reingoldify_helper(node.as_ref().borrow().get_left_child(), node_visitor);
            Self::reingoldify_helper(node.as_ref().borrow().get_right_child(), node_visitor);
        }
    }

    pub fn create_reingold_layout2(&mut self, node_visitor: crate::CBFnNodeVistor2) -> i32 {
        if let Some(root) = &self.clam_root {
            if let Some(labels) = &self.labels {
                let layout_root =
                    reingold_impl::Node::init_draw_tree(&root.as_ref().borrow(), labels);

                let result = Self::reingoldify2(layout_root, node_visitor);
                return result;
            } else {
                return -3;
            }
        } else {
            return -4;
        }
    }

    pub fn reingoldify2(root: reingold_impl::Link, node_visitor: crate::CBFnNodeVistor2) -> i32 {
        if let Some(_) = root.clone() {
            Self::reingoldify_helper2(root.clone(), node_visitor);

            return 1;
        }
        return -1;
    }

    fn reingoldify_helper2(root: reingold_impl::Link, node_visitor: crate::CBFnNodeVistor2) -> () {
        if let Some(node) = root {
            let mut baton = NodeData2::from_reingold_node(&node.as_ref().borrow());

            node_visitor(Some(&baton));
            // unsafe {
            //     NodeFFI::from_ptr(baton).free_ids();
            // }

            baton.free_ids();

            Self::reingoldify_helper2(node.as_ref().borrow().get_left_child(), node_visitor);
            Self::reingoldify_helper2(node.as_ref().borrow().get_right_child(), node_visitor);
        }
    }
}
