// extern crate nalgebra as na;

// use abd_clam::core::cluster::Cluster;
// use abd_clam::core::cluster_criteria::PartitionCriteria;
// use abd_clam::core::dataset::VecVec;
// use std::cell::RefCell;
// use std::ffi::CString;
// use std::mem::transmute;
// use std::rc::Rc;

// use crate::utils::{anomaly_readers, distances, helpers};

// use crate::debug;

// use super::node::NodeData;
// use super::reingold_impl::{self};
// pub type Clusterf32<'a> = Cluster<'a, f32, f32, VecVec<f32, f32>>;
// type DataSet<'a> = VecVec<f32, f32>;

// pub struct Handle<'a> {
//     abd_clam_root: Option<Clusterf32<'a>>,
//     dataset: Option<DataSet<'a>>,
//     labels: Option<Vec<u8>>,
// }

// impl<'a> Handle<'a> {
//     pub fn default() -> Self {
//         Handle {
//             abd_clam_root: None,
//             dataset: None,
//             labels: None,
//         }
//     }

//     // pub fn to_raw(data_name: &str, cardinality: usize) -> *mut Handle<'a> {
//     //     let mut handle = Self::new(data_name, cardinality);
//     //     handle.build_root_with_self(cardinality);
//     //     return Box::into_raw(Box::new(handle));
//     // }

//     pub fn new(data_name: &str, cardinality: usize) -> Handle<'a> {
//         let mut h = Handle::default();
//         let (dataset, labels) = Self::create_dataset(data_name);

//         h.dataset = Some(dataset);
//         h.labels = Some(labels);

//         // if let Some(data) = &h.dataset {
//         //     h.abd_clam_root = Self::build_root(data, cardinality);
//         // }
//         // h.build_root_with_self(cardinality);

//         return h;

//         // h.build_root_with_self(cardinality);
//         // Self::build_root_with_param(&mut h.abd_clam_root.unwrap(), &h.dataset.unwrap(), cardinality);
//         // if let Some(cluster) = &mut h.borrow_mut().abd_clam_root {
//         //     if let Some(data) = &h.as_ref().borrow().dataset {
//         //         Self::build_root_with_param_root(cluster, data, cardinality);
//         //     }
//         // }
//         // return h;

//         // _ = Box::into_raw(Box::new(h));
//     }

//     fn build_root(dataset: &'a DataSet, cardinality: usize) -> Option<Clusterf32<'a>> {
//         let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);

//         return Some(
//             Cluster::new_root(dataset)
//                 .partition(&criteria, true)
//                 .with_seed(1),
//         );
//     }

//     fn build_root_with_param_root(
//         cluster: &'a mut Clusterf32<'a>,
//         dataset: &'a DataSet,
//         cardinality: usize,
//     ) {
//         // if let Some(dataset) = dataset {
//         let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);

//         *cluster = Cluster::new_root(dataset)
//             .partition(&criteria, true)
//             .with_seed(1);
//         // }
//     }

//     fn build_root_with_self(&'a mut self, cardinality: usize) {
//         if let Some(dataset) = &self.dataset {
//             let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);

//             self.abd_clam_root = Some(
//                 Cluster::new_root(dataset)
//                     .partition(&criteria, true)
//                     .with_seed(1),
//             );
//         }
//     }

//     fn create_dataset(data_name: &str) -> (DataSet, Vec<u8>) {
//         let (data, labels) = anomaly_readers::read_anomaly_data(data_name, true).unwrap();
//         let dataset = VecVec::new(data, distances::euclidean_sq, data_name.to_string(), false);
//         return (dataset, labels);
//     }
// }
