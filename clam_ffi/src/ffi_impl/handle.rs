extern crate nalgebra as na;

use std::cell::RefCell;
use std::mem::transmute;
use std::rc::Rc;

use clam::core::cluster::Cluster;
use clam::core::cluster_criteria::PartitionCriteria;
use clam::core::dataset::{self, VecVec};

use crate::utils::{anomaly_readers, distances};

// #[macro_use]
use crate::debug;

use super::node::NodeI;
use super::reingold_impl;
type Clusterf32<'a> = Cluster<'a, f32, f32, VecVec<f32, f32>>;
type DataSet<'a> = VecVec<f32, f32>;

pub struct Handle<'a> {
    clam_root: Clusterf32<'a>,
    dataset: DataSet<'a>,
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

    // pub fn build_clam(dataset: &'a DataSet, cardinality: usize) -> Clusterf32<'a> {
    //     let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);

    //     Cluster::new_root(dataset)
    //         .partition(&criteria, true)
    //         .with_seed(0)
    // }

    pub fn build_clam(&'a mut self, cardinality: usize) -> () {
        let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);
        // if let Some(dataset) = &self.dataset {
            self.clam_root =
                Cluster::new_root(&self.dataset)
                    .partition(&criteria, true)
                    .with_seed(0);
            
        // }
    }

    // pub fn default() -> Self {
    //     Handle {
    //         clam_root: None,
    //         dataset: None,
    //         labels: None,
    //         layout: None,
    //         // data: (None, None),
    //     }
    // }

    pub fn init_dataset(&mut self, data_name: &str) -> i32 {
        match Self::create_dataset(data_name) {
            Ok((data, labels)) => {
                self.dataset = data;
                self.labels = Some(labels);
                // self.build_clam_root(cardinality);
                return 1;
            }
            Err(e) => {
                debug!("{}", e);
                return 0;
            }
        }
    }

    pub fn get_dataset(&self) -> &DataSet {
        &self.dataset
    }

    // pub fn get_root_mut(&mut self) -> &mut Clusterf32<'a> {
    //     self.clam_root.as_mut().unwrap()
    // }

    pub fn init_reingold_layout(handle: &'a mut Handle<'a>) {}
    // let (dataset, labels) = Self::create_dataset(data_name, cardinality).unwrap();

    //     match Self::create_dataset(data_name, cardinality) {
    //         Ok((dataset, labels)) => {
    //             Ok(Handle {
    //                 clam_root: Some(root,
    //                 dataset: dataset,
    //                 labels: labels,
    //                 layout: None,
    //             })
    //         }
    //         Err(e) => Err(e),
    //     }
    // }
    // seeded to 0 for dev testing
    // pub fn build_clam_root(
    //     dataset: &'a DataSet,
    //     root: &'a mut Clusterf32<'a>,
    //     cardinality: usize,
    // ) -> () {
    //     // let dataset = Self::create_dataset(data_name, cardinality);
    //     // match dataset {
    //     //     Ok((ds, labels)) => {
    //     let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);
    //     // if let Some(dataset) = &dataset {
    //     // self.clam_root = None;
    //     // let root1 = Some(
    //     //     Cluster::new_root(dataset)
    //     //         .partition(&criteria, true)
    //     //         .with_seed(0),
    //     // );
    //     *root = Cluster::new_root(dataset)
    //         .partition(&criteria, true)
    //         .with_seed(0);
    // }
    // }
    // if let Some(dataset) = &self.dataset {
    //     let root = Cluster::new_root(dataset);
    //     return Some(root);
    //     // self.clam_root = Some(root);
    // }
    // let root = Cluster::new_root(&dataset.)
    //     .partition(&criteria, true)
    //     .with_seed(0);

    // return root;
    // let (data, labels) = dataset.unwrap();
    //         Ok((root, ds, labels))
    //     }
    //     Err(e) => Err(e),
    // }
    // match dataset
    // {
    //     Ok(dataset2)=> {
    //         let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);

    //         let root = Cluster::new_root(&dataset2)
    //             .partition(&criteria, true)
    //             .with_seed(0);
    //         debug!("finished building clam tree");
    //     },
    //     Err(e) => Err(e)
    // }
    // match anomaly_readers::read_anomaly_data(data_name, false) {
    // Ok((first_data, labels)) => {
    //     debug!("data_name was valid");
    //     let dataset = VecVec::new(
    //         first_data,
    //         distances::euclidean_sq,
    //         data_name.to_string(),
    //         false,
    //     );
    // let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);

    // let root = Cluster::new_root(&dataset)
    //     .partition(&criteria, true)
    //     .with_seed(0);
    // debug!("finished building clam tree");

    // // let draw_node = reingold_impl::Node::example_tree2();
    // // let draw_node = Node::make_complete_tree(10);
    // // let draw_node = reingold_impl::Node::init_draw_tree(&root, &labels);
    // debug!("finished building draw node  tree");

    // return Ok((root, dataset, labels));
    // }
    // Err(e) => {
    //     return Err(e);
    // }
    // }
    // }

    // pub fn new(data_name: &str, cardinality: usize) -> Result<Handle, String> {
    //     match Self::build_clam_root(data_name, cardinality) {
    //         Ok((root, dataset, labels)) => Ok(Handle {
    //             clam_root: root,
    //             dataset: dataset,
    //             labels: labels,
    //             layout: None,
    //         }),
    //         Err(e) => Err(e),
    //     }
    // }

    pub fn create_reingold_layout(&mut self) -> () {
        let layout_root = reingold_impl::Node::example_tree2();
        let layout = reingold_impl::reingold_tree_to_vec(layout_root);
        // let draw_node = Node::make_complete_tree(10);
        // let draw_node = reingold_impl::Node::init_draw_tree(&root, &labels);
        debug!("finished building draw node  tree");

        self.layout = Some(layout);
    }

    pub fn free_reingold_layout(&mut self) {
        self.layout = None;
    }
}
