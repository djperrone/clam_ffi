// need to make struct fields private later

#![allow(dead_code)]
#![allow(unused_variables)]
// use glam::Vec3;

use crate::{debug, utils::helpers};

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
            debug!("warning unity struct was sent with null id");
            self.id = StringFFI::new(node.name());
        }

        if self.left_id.is_empty() {
            debug!("warning unity struct was sent with null lid");

            self.left_id = StringFFI::new(left_id);
        }

        if self.right_id.is_empty() {
            debug!("warning unity struct was sent with null rid");

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

    pub fn from_reingold_node(other: &reingold_impl::Node) -> Self {
        let (left, right) = other.get_child_names();
        NodeData2 {
            pos: glam::Vec3::new(other.get_x(), other.get_y(), 0.),
            color: glam::Vec3::new(
                other.get_color().x,
                other.get_color().y,
                other.get_color().z,
            ),
            id: StringFFI::new(other.get_name()),
            left_id: StringFFI::new(left),
            right_id: StringFFI::new(right),
            cardinality: -1,
            depth: -1,
            arg_center: -1,
            arg_radius: -1,
        }
    }
    pub fn free_ids(&mut self) {
        self.id.free();
        self.left_id.free();
        self.right_id.free();
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct StringFFI {
    pub data: *mut u8,
    pub len: i32,
    pub is_owned_by_unity: bool,
}

impl StringFFI {
    pub fn new(data: String) -> Self {
        StringFFI {
            data: helpers::alloc_to_c_char(data.clone()) as *mut u8,
            len: data.len() as i32,
            is_owned_by_unity: false,
        }
    }

    pub unsafe fn as_string(&self) -> Result<String, String> {
        return helpers::csharp_to_rust_utf8(self.data, self.len);
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

    pub fn free(&mut self) {
        helpers::free_string(self.data);
        self.len = 0;
    }
}
