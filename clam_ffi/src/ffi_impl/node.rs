#![allow(dead_code)]
#![allow(unused_variables)]
// use glam::Vec3;

use crate::utils::helpers;

use super::{
    // glam,
    handle::Clusterf32,
    reingold_impl::{self},
};

#[repr(C)]
pub struct NodeFFI {
    pos: glam::Vec3,   // stored in unity
    color: glam::Vec3, // stored in unity
    id: *mut i8,       // stored in unity and rust
    left_id: *mut i8,  //stored in unity and rust
    right_id: *mut i8, //stored in unity and rust
    cardinality: i32,  // stored in rust
    depth: i32,        // stored in rust
    arg_center: i32,   // stored in rust
    arg_radius: i32,   // stored in rust
}

impl NodeFFI {
    pub fn new(
        pos: glam::Vec3,
        color: glam::Vec3,
        id: String,
        left_id: String,
        right_id: String,
    ) -> Self {
        NodeFFI {
            pos: pos,
            color: color,
            id: helpers::alloc_to_c_char(id),
            left_id: helpers::alloc_to_c_char(left_id),
            right_id: helpers::alloc_to_c_char(right_id),
            cardinality: -1,
            depth: -1,
            arg_center: -1,
            arg_radius: -1,
        }
    }

    pub fn from_clam(node: &Clusterf32) -> Self {
        let (left, right) = {
            if let Some([left, right]) = node.children() {
                (left.name(), right.name())
            } else {
                (String::from("-1"), String::from("-1"))
            }
        };

        NodeFFI {
            pos: glam::Vec3::new(1., 2., 3.),
            color: glam::Vec3::new(0., 0., 0.),
            id: helpers::alloc_to_c_char(node.name()),
            left_id: helpers::alloc_to_c_char(left),
            right_id: helpers::alloc_to_c_char(right),
            cardinality: node.cardinality() as i32,
            depth: node.depth() as i32,
            arg_center: node.arg_center() as i32,
            arg_radius: node.arg_radius() as i32,
        }
    }
    pub fn new_ids(id: String, left: String, right: String) -> Self {
        NodeFFI {
            pos: glam::Vec3::new(0., 0., 0.),
            color: glam::Vec3::new(0., 0., 0.),
            id: helpers::alloc_to_c_char(id),
            left_id: helpers::alloc_to_c_char(left),
            right_id: helpers::alloc_to_c_char(right),
            cardinality: -1,
            depth: -1,
            arg_center: -1,
            arg_radius: -1,
        }
    }

    pub fn from_reingold_node(node: &reingold_impl::Node) -> Self {
        let color = node.get_color();
        let child_names = node.get_child_names();
        NodeFFI {
            pos: glam::Vec3::new(node.get_x(), node.get_y(), 0.),
            color: glam::Vec3::new(color.x, color.y, color.z),
            id: helpers::alloc_to_c_char(node.get_name()),
            left_id: helpers::alloc_to_c_char(child_names.0),
            right_id: helpers::alloc_to_c_char(child_names.1),
            cardinality: -1,
            depth: -1,
            arg_center: -1,
            arg_radius: -1,
        }
    }
    pub fn free_ids(&mut self) {
        helpers::free_c_char(self.id);
        helpers::free_c_char(self.left_id);
        helpers::free_c_char(self.right_id);
    }

    // pub fn set_pos(&self, x: f32, y: f32, z: f32) {
    //     self.p
    // }

    pub fn to_ptr(self) -> *mut NodeFFI {
        Box::into_raw(Box::new(self))
    }

    pub unsafe fn from_ptr(ptr: *mut NodeFFI) -> Box<NodeFFI> {
        Box::from_raw(ptr)
    }
}

// use std::{
//     ffi::{c_char, CString},
//     ptr::{null, null_mut},
// };

// use super::{
//     handle::Clusterf32,
//     reingold_impl::{self, Node},
// };
// use crate::utils::helpers;
// extern crate nalgebra as na;
// use glam::Vec3;

// #[repr(C)]
// pub struct NodeTest {
//     pos: glam::Vec3,
//     r: f32,
// }

// #[repr(C)]
// pub struct NodeBaton {
//     x: f32,
//     y: f32,
//     z: f32,
//     r: f32,
//     g: f32,
//     b: f32,
//     id: *mut i8,
//     left: *mut i8,
//     right: *mut i8,
// }

// impl NodeBaton {
//     pub fn new_names(id: String, left_id: String, right_id: String) -> Self {
//         // let test = left_id.as_mut_ptr() as *mut i8;
//         NodeBaton {
//             x: 0.,
//             y: 0.,
//             z: 0.,
//             r: 0.,
//             g: 0.,
//             b: 0.,
//             id: helpers::alloc_to_c_char(id),
//             left: helpers::alloc_to_c_char(left_id),
//             right: helpers::alloc_to_c_char(right_id),
//         }
//     }

//     pub fn new_from_reingold_node(node: &reingold_impl::Node) -> Self {
//         // let (left_name, right_name) = {
//         //     if !node.is_leaf() {
//         //         (
//         //             node.get_left_child().unwrap().as_ref().borrow().get_name(),
//         //             node.get_right_child().unwrap().as_ref().borrow().get_name(),
//         //         )
//         //     } else {
//         //         (String::new(), String::new())
//         //     }
//         // };
//         let color = node.get_color();
//         let (left_name, right_name) = node.get_child_names();
//         NodeBaton {
//             x: node.get_x(),
//             y: node.get_y(),
//             z: 0.,
//             r: color.x,
//             g: color.y,
//             b: color.z,
//             id: helpers::alloc_to_c_char(node.get_name()),
//             left: helpers::alloc_to_c_char(left_name),
//             right: helpers::alloc_to_c_char(right_name),
//             // id: null_mut() as *mut i8,
//             // left: null_mut() as *mut i8,
//             // right: null_mut() as *mut i8,
//             // id: CString::new(node.get_name()).expect("").into_raw(),
//             // left: CString::new(left_name).expect("").into_raw(),
//             // right: CString::new(right_name).expect("").into_raw(),
//         }

//         // NodeBaton {
//         //     x: node.pos.x,
//         //     y: node.pos.y,
//         //     z: node.pos.z,
//         //     r: node.color.x,
//         //     g: node.color.y,
//         //     b: node.color.z,
//         //     id: CString::new(node.id.clone()).expect("").into_raw(),
//         //     left: CString::new(node.id.clone()).expect("").into_raw(),
//         //     right: CString::new(node.id.clone()).expect("").into_raw(),
//         // }
//     }
//     pub fn free_string_names(&mut self) -> () {
//         helpers::free_c_char(self.id);
//         helpers::free_c_char(self.left);
//         helpers::free_c_char(self.right);
//     }
//     // pub fn set_ids(&mut self, id: *mut i8, left: *mut i8, right: *mut i8) {
//     //     self.id = id;
//     //     self.left = left;
//     //     self.right = right;
//     // }

//     pub fn to_ptr(self) -> *mut NodeBaton {
//         Box::into_raw(Box::new(self))
//     }

//     pub fn free(ptr: *mut NodeBaton) {
//         if ptr.is_null() {
//             return;
//         }
//         unsafe {
//             let b = Box::from_raw(ptr);
//             // let test = CString::from_raw(b.id as *mut c_char);
//             // let test2 = CString::from_raw(b.left as *mut c_char);
//             // let test3 = CString::from_raw(b.right as *mut c_char);
//         }
//     }
// }

// pub struct NodeI {
//     pos: na::Vector3<f32>,
//     color: na::Vector3<f32>,
//     id: String,
//     left_child: String,
//     right_child: String,
// }

// impl NodeI {
//     pub fn new(
//         x: f32,
//         y: f32,
//         z: f32,
//         r: f32,
//         g: f32,
//         b: f32,
//         id: String,
//         left: String,
//         right: String,
//     ) -> Self {
//         NodeI {
//             pos: na::Vector3::new(x, y, z),
//             color: (na::Vector3::new(r, g, b)),
//             id: id,
//             left_child: left,
//             right_child: right,
//         }
//     }

//     // pub unsafe fn from_baton(baton: NodeBaton) -> Self {
//     //     return Self::new(
//     //         baton.x,
//     //         baton.y,
//     //         baton.z,
//     //         baton.r,
//     //         baton.g,
//     //         baton.b,
//     //         String::from(CString::from_raw(baton.id).to_str()),
//     //         baton.left,
//     //         baton.right,
//     //     );
//     // }

//     pub fn get_pos(&self) -> &na::Vector3<f32> {
//         &self.pos
//     }

//     pub fn get_color(&self) -> &na::Vector3<f32> {
//         &self.color
//     }

//     pub fn get_left_child(&self) -> &String {
//         &self.left_child
//     }

//     pub fn get_right_child(&self) -> &String {
//         &self.right_child
//     }
//     pub fn get_id(&self) -> &String {
//         &self.id
//     }
// }
