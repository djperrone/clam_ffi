extern crate nalgebra as na;

use clam::core::cluster::Cluster;
use clam::core::cluster_criteria::PartitionCriteria;
use clam::core::dataset::VecVec;
use std::cell::RefCell;
use std::mem::transmute;
use std::rc::Rc;

use crate::utils::{anomaly_readers, distances};

use crate::debug;

use super::node::NodeI;
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

    pub fn set_root(&'a mut self, root: Rc<RefCell<Clusterf32<'a>>>) {
        self.clam_root = Some(root.clone());
    }
    pub fn get_dataset(&self) -> &DataSet {
        &self.dataset.as_ref().unwrap()
    }
    pub fn get_layout(&self) -> &Option<Vec<NodeI>> {
        return &self.layout;
    }

    pub fn create_reingold_layout(&mut self) {
        let layout_root = reingold_impl::Node::example_tree2();
        // let layout = reingold_impl::reingold_tree_to_vec(layout_root);
        // let layout = reingold_impl::reingold_tree_to_vec(layout_root);
        // let draw_node = Node::make_complete_tree(10);
        // let draw_node = reingold_impl::Node::init_draw_tree(
        //     &self.clam_root.as_ref().unwrap().as_ref().borrow(),
        //     self.labels.as_ref().unwrap(),
        // );
        debug!("finished building draw node  tree");
        let layout = reingold_impl::flatten_reingold_tree(layout_root);
        // let layout = reingold_impl::reingold_tree_to_vec(draw_node);

        self.layout = Some(layout);
        debug!("layout set");
    }

    pub fn free_reingold_layout(&mut self) {
        self.layout = None;
    }
}
