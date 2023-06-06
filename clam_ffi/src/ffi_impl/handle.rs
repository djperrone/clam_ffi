extern crate nalgebra as na;

use clam::core::cluster::Cluster;
use clam::core::cluster_criteria::PartitionCriteria;
use clam::core::dataset::VecVec;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::mem::transmute;
use std::rc::Rc;

use crate::utils::{anomaly_readers, distances, helpers};

use crate::debug;

use super::node::{NodeBaton, NodeI};
use super::reingold_impl;
type Clusterf32<'a> = Cluster<'a, f32, f32, VecVec<f32, f32>>;
type DataSet<'a> = VecVec<f32, f32>;

pub struct Handle<'a> {
    clam_root: Option<Rc<RefCell<Clusterf32<'a>>>>,
    dataset: Option<DataSet<'a>>,
    labels: Option<Vec<u8>>,
    layout: Option<Vec<NodeI>>,
    // data: (Option<DataSet<'a>>, Option<Clusterf32<'a>>),
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
            layout: None,
        }
    }
    fn create_dataset(data_name: &str) -> Result<(DataSet, Vec<u8>), String> {
        match anomaly_readers::read_anomaly_data(data_name, false) {
            Ok((first_data, labels)) => {
                // debug!("data_name was valid");
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
                    .with_seed(0),
            )));
            // return Err("invalid dataset".to_string());
        } else {
            return Err("invalid dataset".to_string());
        }

        return Err("invalid dataset".to_string());
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
    pub fn get_layout(&self) -> &Option<Vec<NodeI>> {
        return &self.layout;
    }

    pub fn create_reingold_layout(&mut self, send_baton: crate::CBFnNodeBaton) -> i32 {
        // let layout_root = reingold_impl::Node::example_tree2();
        // let layout = reingold_impl::reingold_tree_to_vec(layout_root);
        // let layout = reingold_impl::reingold_tree_to_vec(layout_root);
        // let draw_node = Node::make_complete_tree(10);

        if let Some(root) = &self.clam_root {
            if let Some(labels) = &self.labels {
                let layout_root =
                    reingold_impl::Node::init_draw_tree(&root.as_ref().borrow(), labels);

                let result = Self::dfs_callback_set_positions(layout_root, send_baton);
                return result;
            } else {
                return -3;
            }
        } else {
            return -4;
        }
        // debug!("finished building draw node  tree");
        // let layout = reingold_impl::flatten_reingold_tree(layout_root);
        // let layout = reingold_impl::reingold_tree_to_vec(draw_node);

        // self.layout = Some(layout);\
        return -5;
        // debug!("layout set");
        // return result;
        // return 0;
    }

    pub fn dfs_callback_set_positions(
        root: reingold_impl::Link,
        send_baton: crate::CBFnNodeBaton,
    ) -> i32 {
        if let Some(_) = root.clone() {
            Self::dfs_callback_set_positions_helper(root.clone(), send_baton);

            return 1;
        }

        return -1;
    }

    fn dfs_callback_set_positions_helper(
        root: reingold_impl::Link,
        send_baton: crate::CBFnNodeBaton,
    ) -> () {
        if let Some(node) = root {
            let baton = NodeBaton::new_from_reingold_node(&node.as_ref().borrow());
            // let id = helpers::alloc_to_c_char(node.as_ref().borrow().get_name());
            // let (left, right) = node.as_ref().borrow().get_child_names();
            // let left_ptr = helpers::alloc_to_c_char(left);
            // let right_ptr = helpers::alloc_to_c_char(right);
            let id = helpers::alloc_to_c_char(node.as_ref().borrow().get_name());
            // baton.set_ids(id.clone(), left_ptr, right_ptr);
            let ptr = baton.to_ptr();
            send_baton(ptr, id);
            NodeBaton::free(ptr);
            helpers::free_c_char(id);
            // helpers::free_c_char(left_ptr);
            // helpers::free_c_char(right_ptr);

            Self::dfs_callback_set_positions_helper(
                node.as_ref().borrow().get_left_child(),
                send_baton,
            );
            Self::dfs_callback_set_positions_helper(
                node.as_ref().borrow().get_right_child(),
                send_baton,
            );
        }
    }

    pub fn free_reingold_layout(&mut self) {
        self.layout = None;
    }

    pub fn dfs_callback_set_node_names(&self, set_csharp_node_id: crate::CBFnInitNode) -> i32 {
        if let Some(root) = &self.clam_root {
            let node = root.as_ref().borrow();
            Self::dfs_callback_set_node_names_helper(&node, set_csharp_node_id);
            return 1;
        }
        return 1;
        // return 0;
    }

    fn dfs_callback_set_node_names_helper(
        root: &Clusterf32,
        set_csharp_node_id: crate::CBFnInitNode,
    ) -> () {
        if root.is_leaf() {
            let id = helpers::alloc_to_c_char(root.name());
            let lid = helpers::alloc_to_c_char(String::from(""));
            let rid = helpers::alloc_to_c_char(String::from(""));
            set_csharp_node_id(id, lid, rid);
            helpers::free_c_char(id);
            helpers::free_c_char(lid);
            helpers::free_c_char(rid);
            return;
        }

        let children = root.children();
        if let Some([left, right]) = children {
            let id = helpers::alloc_to_c_char(root.name());
            let lid = helpers::alloc_to_c_char(left.name());
            let rid = helpers::alloc_to_c_char(right.name());

            set_csharp_node_id(id, lid, rid);

            helpers::free_c_char(id);
            helpers::free_c_char(lid);
            helpers::free_c_char(rid);

            Self::dfs_callback_set_node_names_helper(left, set_csharp_node_id);
            Self::dfs_callback_set_node_names_helper(right, set_csharp_node_id);
        }
    }
}
