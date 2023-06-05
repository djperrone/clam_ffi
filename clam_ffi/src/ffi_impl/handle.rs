extern crate nalgebra as na;

use std::cell::RefCell;
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
    clam_root: Option<Clusterf32<'a>>,
    dataset: Option<DataSet<'a>>,
    labels: Option<Vec<u8>>,
    layout: Option<Vec<NodeI>>,
}

impl<'a> Handle<'a> {
    fn create_dataset(data_name: &str, cardinality: usize) -> Result<(DataSet, Vec<u8>), String> {
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

    pub fn default() -> Self {
        Handle {
            clam_root: None,
            dataset: None,
            labels: None,
            layout: None,
        }
    }

    pub fn init(handle: &'a mut Handle<'a>, data_name: &str, cardinality: usize) -> () {
        match Self::create_dataset(data_name, cardinality) {
            Ok((data, labels)) => {
                handle.dataset = Some(data);
                if let Some(dataset) = &handle.dataset {
                    let root = Self::build_clam_root(&dataset, cardinality);
                    handle.labels = Some(labels);
                    handle.clam_root = Some(root);
                }
            }
            Err(e) => debug!("{}", e),
        }
    }
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
    fn build_clam_root(dataset: &'a DataSet, cardinality: usize) -> Clusterf32<'a> {
        // let dataset = Self::create_dataset(data_name, cardinality);
        // match dataset {
        //     Ok((ds, labels)) => {
        let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);
        let root = Cluster::new_root(dataset)
            .partition(&criteria, true)
            .with_seed(0);

        return root;
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
    }

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
        let layout = reingold_impl::reingold_tree_to_vec(layout_root.clone());
        // let draw_node = Node::make_complete_tree(10);
        // let draw_node = reingold_impl::Node::init_draw_tree(&root, &labels);
        debug!("finished building draw node  tree");

        self.layout = Some(layout);
    }

    pub fn free_reingold_layout(&mut self) {
        self.layout = None;
    }
}
