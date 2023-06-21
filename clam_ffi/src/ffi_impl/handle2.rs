// extern crate nalgebra as na;

// use clam::core::cluster::Cluster;
// use clam::core::cluster_criteria::PartitionCriteria;
// use clam::core::dataset::VecVec;
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
//     clam_root: Option<Rc<RefCell<Clusterf32<'a>>>>,
//     dataset: Option<DataSet<'a>>,
//     labels: Option<Vec<u8>>,
// }

// impl<'a> Handle<'a> {
//     pub fn to_ptr(self) -> *mut Handle<'a> {
//         unsafe { transmute(Box::new(self)) }
//     }

//     pub fn from_ptr(ptr: *mut Handle) -> &'a mut Handle {
//         unsafe { &mut *ptr }
//     }
//     pub fn default() -> Self {
//         Handle {
//             clam_root: None,
//             dataset: None,
//             labels: None,
//         }
//     }
//     fn create_dataset(data_name: &str) -> Result<(DataSet, Vec<u8>), String> {
//         match anomaly_readers::read_anomaly_data(data_name, false) {
//             Ok((first_data, labels)) => {
//                 let dataset = VecVec::new(
//                     first_data,
//                     distances::euclidean_sq,
//                     data_name.to_string(),
//                     false,
//                 );

//                 Ok((dataset, labels))
//             }
//             Err(e) => Err(e),
//         }
//     }

//     pub fn build_clam(&mut self, cardinality: usize) -> Result<Rc<RefCell<Clusterf32>>, String> {
//         let criteria = PartitionCriteria::new(true).with_min_cardinality(cardinality);
//         if let Some(dataset) = &self.dataset {
//             return Ok(Rc::new(RefCell::new(
//                 Cluster::new_root(dataset)
//                     .partition(&criteria, true)
//                     .with_seed(1),
//             )));
//         } else {
//             return Err("invalid dataset".to_string());
//         }
//     }

//     pub fn init_dataset(&mut self, data_name: &str) -> i32 {
//         debug!("dataname in init_Dataset {}", data_name);
//         match Self::create_dataset(data_name) {
//             Ok((data, labels)) => {
//                 self.dataset = Some(data);
//                 self.labels = Some(labels);
//                 return 1;
//             }
//             Err(e) => {
//                 debug!("{}", e);
//                 return 0;
//             }
//         }
//     }

//     pub fn for_each_dft(
//         &mut self,
//         node_visitor: crate::CBFnNodeVistor2,
//         start_node: String,
//     ) -> i32 {
//         if let Some(root) = &self.clam_root {
//             if start_node == "root" {
//                 let node = root.as_ref().borrow();
//                 Self::for_each_dft_helper(&node, node_visitor);
//                 return 1;
//             } else {
//                 //let start_node = find for fucks sake; but no lifetimes existl
//             }
//         }
//         return 0;
//     }

//     fn for_each_dft_helper(root: &Clusterf32, node_visitor: crate::CBFnNodeVistor2) {
//         if root.is_leaf() {
//             let mut baton = NodeData::from_clam(&root);

//             node_visitor(Some(&baton));
//             baton.free_ids();
//             return;
//         }
//         if let Some([left, right]) = root.children() {
//             let mut baton = NodeData::from_clam(&root);

//             node_visitor(Some(&baton));

//             baton.free_ids();

//             Self::for_each_dft_helper(left, node_visitor);
//             Self::for_each_dft_helper(right, node_visitor);
//         }
//     }

//     pub fn get_num_nodes(&self) -> i32 {
//         if let Some(root) = &self.clam_root {
//             root.as_ref().borrow().num_descendants() as i32
//         } else {
//             0
//         }
//     }
//     pub fn set_root(&'a mut self, root: Rc<RefCell<Clusterf32<'a>>>) {
//         self.clam_root = Some(root.clone());
//     }
//     pub fn get_dataset(&self) -> &DataSet {
//         &self.dataset.as_ref().unwrap()
//     }

//     pub fn get_root(&self) -> &Option<Rc<RefCell<Clusterf32<'a>>>> {
//         &self.clam_root
//     }

//     pub unsafe fn test_find(&self) -> &'a Clusterf32<'a> {
//         if let Some(root) = self.clam_root.clone() {
//             if let Some([left, right]) = root.as_ptr().as_mut().unwrap().children() {
//                 return left;
//             } else {
//             }
//         }
//         panic!();
//     }

//     pub unsafe fn find_node(&self, path: String) -> Result<&'a Clusterf32<'a>, String> {
//         if let Some(root) = self.clam_root.clone() {
//             let mut path: String = helpers::hex_to_binary(path)
//                 .trim_start_matches('0')
//                 .chars()
//                 .rev()
//                 .collect();
//             path.pop();

//             return Self::find_node_helper(root.as_ptr().as_mut().unwrap(), path);
//         }
//         debug!("root not built");
//         return Err("root not built".to_string());
//     }

//     pub fn find_node_helper(
//         root: &'a Clusterf32,
//         mut path: String,
//     ) -> Result<&'a Clusterf32<'a>, String> {
//         if path.len() == 0 {
//             return Ok(&root);
//         }
//         let choice: char = path.pop().unwrap();
//         if let Some([left, right]) = root.children() {
//             if choice == '0' {
//                 return Self::find_node_helper(left, path);
//             } else if choice == '1' {
//                 return Self::find_node_helper(right, path);
//             } else {
//                 return Err("invalid character in node name".to_string());
//             }
//         } else {
//             return Err("node not found - no children available".to_string());
//         }
//     }

//     // pub fn find_node(&self, path: String) -> Result<NodeData, String> {
//     //     if let Some(root) = self.clam_root.clone() {
//     //         let root = root.as_ref().borrow();
//     //         // debug!(
//     //         //     "searching for (node name in hex): {}",
//     //         //     // unity_node.id.as_string()
//     //         // );
//     //         let mut path: String = helpers::hex_to_binary(path)
//     //             .trim_start_matches('0')
//     //             .chars()
//     //             .rev()
//     //             .collect();
//     //         path.pop();

//     //         let out = Self::find_node_helper(
//     //             &root,
//     //             // clam_helpers:: unity_node.id.as_string().chars().rev().collect(),
//     //             path,
//     //         );

//     //         return out;
//     //     }
//     //     debug!("root not built");
//     //     return Err("root not built".to_string());
//     // }

//     // pub fn find_node_helper(root: &Clusterf32, mut path: String) -> Result<NodeData, String> {
//     //     if path.len() == 0 {
//     //         return Ok(NodeData::from_clam(root));
//     //     }
//     //     let choice: char = path.pop().unwrap();
//     //     if let Some([left, right]) = root.children() {
//     //         if choice == '0' {
//     //             return Self::find_node_helper(left, path);
//     //         } else if choice == '1' {
//     //             return Self::find_node_helper(right, path);
//     //         } else {
//     //             return Err("invalid character in node name".to_string());
//     //         }
//     //     } else {
//     //         return Err("node not found - no children available".to_string());
//     //     }
//     // }

//     pub fn create_reingold_layout(&mut self, node_visitor: crate::CBFnNodeVistor2) -> i32 {
//         if let Some(root) = &self.clam_root {
//             if let Some(labels) = &self.labels {
//                 let layout_root =
//                     reingold_impl::Node::init_draw_tree(&root.as_ref().borrow(), labels);

//                 let result = Self::reingoldify(layout_root, node_visitor);
//                 return result;
//             } else {
//                 return -3;
//             }
//         } else {
//             return -4;
//         }
//     }

//     pub fn reingoldify(root: reingold_impl::Link, node_visitor: crate::CBFnNodeVistor2) -> i32 {
//         if let Some(_) = root.clone() {
//             Self::reingoldify_helper(root.clone(), node_visitor);

//             return 1;
//         }
//         return -1;
//     }

//     fn reingoldify_helper(root: reingold_impl::Link, node_visitor: crate::CBFnNodeVistor2) -> () {
//         if let Some(node) = root {
//             let mut baton = NodeData::from_reingold_node(&node.as_ref().borrow());

//             node_visitor(Some(&baton));
//             baton.free_ids();

//             Self::reingoldify_helper(node.as_ref().borrow().get_left_child(), node_visitor);
//             Self::reingoldify_helper(node.as_ref().borrow().get_right_child(), node_visitor);
//         }
//     }
// }
