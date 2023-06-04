extern crate nalgebra as na;

use std::cell::RefCell;
use std::rc::Rc;

use clam::core::cluster::Cluster;
use clam::core::cluster_criteria::PartitionCriteria;
use clam::core::dataset::VecVec;

use crate::utils::{anomaly_readers, distances};

// #[macro_use]
use crate::debug;

use super::reingold_impl;
type Clusterf32<'a> = Cluster<'a, f32, f32, VecVec<f32, f32>>;
type DataSet = VecVec<f32, f32>;

pub struct Handle<'a> {
    clam_root: Option<Clusterf32<'a>>,
    dataset: DataSet,
}

impl<'a> Handle<'a> {
    pub fn init_reingold_layout(
        data_name: &str,
        cardinality: usize,
    ) -> Result<Option<Rc<RefCell<reingold_impl::Node>>>, String> {
        debug!("pos_layout name {}", data_name);

        match anomaly_readers::read_anomaly_data(data_name, false) {
            Ok((first_data, labels)) => {
                debug!("data_name was valid");
                let second_data = VecVec::new(
                    first_data,
                    distances::euclidean_sq,
                    data_name.to_string(),
                    false,
                );
                let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);

                let root = Cluster::new_root(&second_data).par_partition(&criteria, true);
                debug!("finished building clam tree");

                let draw_node = reingold_impl::Node::example_tree2();
                // let draw_node = Node::make_complete_tree(10);
                // let draw_node = reingold_impl::Node::init_draw_tree(&root, &labels);
                debug!("finished building draw node  tree");

                return Ok(draw_node);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
