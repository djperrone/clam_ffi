// need to make struct fields private later

#![allow(dead_code)]
#![allow(unused_variables)]
// use glam::Vec3;

use std::{cell::RefCell, ffi::c_char, ptr::null_mut, rc::Rc};

use crate::utils::helpers;

use super::{
    // glam,
    handle::Clusterf32,
    reingold_impl::{self},
};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NodeData2 {
    pub pos: glam::Vec3,
    pub color: glam::Vec3,

    pub id: StringFFI,
    pub left_id: StringFFI,
    pub right_id: StringFFI,

    pub cardinality: i32,
    pub depth: i32,
    pub arg_center: i32,
    pub arg_radius: i32,
}

impl NodeData2 {
    pub fn new(id: String) -> Self {
        NodeData2 {
            id: StringFFI::new(id),
            color: glam::Vec3::new(0., 0., 0.),
            pos: glam::Vec3::new(0., 0., 0.),
            left_id: StringFFI::new("default".to_string()),
            right_id: StringFFI::new("default".to_string()),
            cardinality: -1,
            depth: -1,
            arg_center: -1,
            arg_radius: -1,
        }
    }

    pub fn set_position(&mut self, pos: glam::Vec3) -> () {
        self.pos = pos;
    }

    pub fn set_color(&mut self, color: glam::Vec3) -> () {
        self.color = color;
    }

    pub fn set_from_clam(&mut self, node: &Clusterf32) -> () {
        let (left_id, right_id) = {
            if let Some([left, right]) = node.children() {
                (left.name(), right.name())
            } else {
                ("default".to_string(), "default".to_string())
            }
        };
        if self.id.is_empty() {
            self.id = StringFFI::new(node.name());
        }

        if self.left_id.is_empty() {
            self.left_id = StringFFI::new(left_id);
        }

        if self.right_id.is_empty() {
            self.right_id = StringFFI::new(right_id);
        }

        self.cardinality = node.cardinality() as i32;
        self.depth = node.depth() as i32;
        self.arg_center = node.arg_center() as i32;
        self.arg_radius = node.arg_radius() as i32;
    }

    pub fn from_clam(node: &Clusterf32) -> Self {
        let (left_id, right_id) = {
            if let Some([left, right]) = node.children() {
                (left.name(), right.name())
            } else {
                ("default".to_string(), "default".to_string())
            }
        };
        // if self.id.is_empty() {
        //     self.id = StringFFI::new(node.name());
        // }

        // if self.left_id.is_empty() {
        //     self.left_id = StringFFI::new(left_id);
        // }

        // if self.right_id.is_empty() {
        //     self.right_id = StringFFI::new(right_id);
        // }

        // self.cardinality = node.cardinality() as i32;
        // self.depth = node.depth() as i32;
        // self.arg_center = node.arg_center() as i32;
        // self.arg_radius = node.arg_radius() as i32;

        NodeData2 {
            pos: glam::Vec3::new(0., 0., 0.),
            color: glam::Vec3::new(0., 0., 0.),
            id: (StringFFI::new(node.name())),
            left_id: StringFFI::new(left_id),
            right_id: StringFFI::new(right_id),
            cardinality: (node.cardinality() as i32),
            depth: (node.depth() as i32),
            arg_center: (node.arg_center() as i32),
            arg_radius: (node.arg_radius() as i32),
        }
    }

    pub fn free_ids(&mut self) {
        helpers::free_string(self.id.as_mut_ptr());
        helpers::free_string(self.left_id.as_mut_ptr());
        helpers::free_string(self.right_id.as_mut_ptr());
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct StringFFI {
    pub data: *mut u8,
    pub len: i32,
}

impl StringFFI {
    pub fn new(data: String) -> Self {
        StringFFI {
            data: helpers::alloc_to_c_char(data.clone()) as *mut u8,
            len: data.len() as i32,
        }
    }

    pub fn as_string(&self) -> String {
        return format!("{:?}", self.data);
    }

    pub fn as_ptr(&self) -> *const u8 {
        return self.data;
    }

    // dont think this works as intended...
    pub fn as_mut_ptr(&self) -> *mut u8 {
        return self.data;
    }

    pub fn is_empty(&self) -> bool {
        return self.data.is_null();
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct NodeData {
    pub pos: glam::Vec3,   // stored in unity
    pub color: glam::Vec3, // stored in unity
    pub id: String,        // stored in unity and rust
    pub left_id: String,   //stored in unity and rust
    pub right_id: String,  //stored in unity and rust
    pub cardinality: i32,  // stored in rust
    pub depth: i32,        // stored in rust
    pub arg_center: i32,   // stored in rust
    pub arg_radius: i32,   // stored in rust
    pub id_len: i32,
}

// impl NodeData {
//     pub fn from_ffi(other: &NodeFFI) -> Self {
//         NodeData {
//             pos: other.pos.clone(),
//             color: other.color.clone(),
//             id: (),
//             left_id: (),
//             right_id: (),
//             cardinality: (),
//             depth: (),
//             arg_center: (),
//             arg_radius: (),
//             id_len: (),
//         }
//     }
// }

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NodeFFI {
    pub pos: glam::Vec3,       // stored in unity
    pub color: glam::Vec3,     // stored in unity
    pub id: *mut c_char,       // stored in unity and rust
    pub left_id: *mut c_char,  //stored in unity and rust
    pub right_id: *mut c_char, //stored in unity and rust
    pub cardinality: i32,      // stored in rust
    pub depth: i32,            // stored in rust
    pub arg_center: i32,       // stored in rust
    pub arg_radius: i32,       // stored in rust
    pub id_len: i32,
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
            id_len: id.len() as i32,
            id: helpers::alloc_to_c_char(id),
            left_id: helpers::alloc_to_c_char(left_id),
            right_id: helpers::alloc_to_c_char(right_id),
            cardinality: -1,
            depth: -1,
            arg_center: -1,
            arg_radius: -1,
        }
    }

    pub fn set_from_node_ffi(&mut self, node: &NodeFFI) -> () {
        self.pos = glam::Vec3::new(1., 2., 3.);
        self.color = glam::Vec3::new(0., 0., 0.);
        self.id_len = 0;
        self.id = null_mut();
        self.left_id = null_mut();
        self.right_id = null_mut();
        self.cardinality = node.cardinality as i32;
        self.depth = node.depth as i32;
        self.arg_center = node.arg_center as i32;
        self.arg_radius = node.arg_radius as i32;
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
            id_len: node.name().len() as i32,
            id: helpers::alloc_to_c_char(node.name()),
            left_id: helpers::alloc_to_c_char(left),
            right_id: helpers::alloc_to_c_char(right),
            cardinality: node.cardinality() as i32,
            depth: node.depth() as i32,
            arg_center: node.arg_center() as i32,
            arg_radius: node.arg_radius() as i32,
        }
    }

    pub fn set_from_clam_node(&mut self, node: Rc<RefCell<Clusterf32>>) -> () {
        let node = node.as_ref().borrow();
        let (left, right) = {
            if let Some([left, right]) = node.children() {
                (left.name(), right.name())
            } else {
                (String::from("-1"), String::from("-1"))
            }
        };

        // self.pos = glam::Vec3::new(1., 2., 3.);
        // self.color = glam::Vec3::new(0., 0., 0.);
        self.id = helpers::alloc_to_c_char(node.name());
        self.left_id = helpers::alloc_to_c_char(left);
        self.right_id = helpers::alloc_to_c_char(right);
        self.cardinality = node.cardinality() as i32;
        self.depth = node.depth() as i32;
        self.arg_center = node.arg_center() as i32;
        self.arg_radius = node.arg_radius() as i32;
    }
    pub fn new_ids(id: String, left: String, right: String) -> Self {
        NodeFFI {
            pos: glam::Vec3::new(0., 0., 0.),
            color: glam::Vec3::new(0., 0., 0.),
            id_len: id.len() as i32,
            id: helpers::alloc_to_c_char(id),
            left_id: helpers::alloc_to_c_char(left),
            right_id: helpers::alloc_to_c_char(right),
            cardinality: -1,
            depth: -1,
            arg_center: -1,
            arg_radius: -1,
        }
    }

    pub fn default() -> Self {
        NodeFFI {
            pos: glam::Vec3::new(0., 0., 0.),   // stored in unity
            color: glam::Vec3::new(0., 0., 0.), // stored in unity
            id_len: "default".to_string().len() as i32,
            id: helpers::alloc_to_c_char("default".to_string()), // stored in unity and rust
            left_id: helpers::alloc_to_c_char("default".to_string()), //stored in unity and rust
            right_id: helpers::alloc_to_c_char("default".to_string()), //stored in unity and rust
            cardinality: 0,                                      // stored in rust
            depth: 0,                                            // stored in rust
            arg_center: 0,                                       // stored in rust
            arg_radius: 0,                                       // stored in rust
        }
    }

    pub fn from_reingold_node(node: &reingold_impl::Node) -> Self {
        let color = node.get_color();
        let child_names = node.get_child_names();
        NodeFFI {
            pos: glam::Vec3::new(node.get_x(), node.get_y(), 0.),
            color: glam::Vec3::new(color.x, color.y, color.z),
            id_len: node.get_name().len() as i32,
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
        if self.id != null_mut() {
            helpers::free_c_char(self.id);
        }
        if self.left_id != null_mut() {
            helpers::free_c_char(self.left_id);
        }
        if self.right_id != null_mut() {
            helpers::free_c_char(self.right_id);
        }
    }

    pub fn from_copy(&mut self, other: &NodeFFI) -> () {
        self.arg_center = other.arg_center;
        self.arg_radius = other.arg_radius;
        self.cardinality = other.cardinality;
        self.depth = other.depth;
        self.id = other.id;
        self.left_id = other.left_id;
        self.right_id = other.right_id;
    }

    pub fn name(&self) -> *mut i8 {
        return self.id;
    }

    pub fn to_ptr(self) -> *mut NodeFFI {
        Box::into_raw(Box::new(self))
    }

    pub unsafe fn from_ptr(ptr: *mut NodeFFI) -> Box<NodeFFI> {
        Box::from_raw(ptr)
    }
}
