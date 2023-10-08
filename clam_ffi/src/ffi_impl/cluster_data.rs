// need to make struct fields private later

#![allow(dead_code)]
#![allow(unused_variables)]
// use glam::Vec3;

use crate::{debug, utils::types::Clusterf32};

use crate::tree_layout::reingold_impl;

use super::string_ffi::StringFFI;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ClusterData {
    pub pos: glam::Vec3,
    pub color: glam::Vec3,

    pub id: StringFFI,
    pub left_id: StringFFI,
    pub right_id: StringFFI,

    pub cardinality: i32,
    pub depth: i32,
    pub radius: f32,
    pub lfd: f32,
    pub arg_center: i32,
    pub arg_radius: i32,

    pub dist_to_query: f32,
}

impl ClusterData {
    pub fn from_physics(id: String, position: glam::Vec3) -> Self {
        ClusterData {
            id: StringFFI::new(id),
            color: glam::Vec3::new(0., 0., 0.),
            pos: position,
            left_id: StringFFI::new("default".to_string()),
            right_id: StringFFI::new("default".to_string()),
            cardinality: -1,
            depth: -1,
            radius: -1.0,
            lfd: -1.0,
            arg_center: -1,
            arg_radius: -1,
            dist_to_query: -1f32,
        }
    }

    pub fn set_left_id(&mut self, id: String) {
        self.left_id.free();
        self.left_id = StringFFI::new(id);
    }

    pub unsafe fn get_id(&self) -> String {
        self.id.as_string().unwrap()
    }

    pub unsafe fn get_ffi_id(&self) -> &StringFFI {
        &self.id
    }
    pub fn set_position(&mut self, pos: glam::Vec3) -> () {
        self.pos = pos;
    }

    pub fn set_color(&mut self, color: glam::Vec3) -> () {
        self.color = color;
    }

    pub fn set_from_clam(&mut self, node: &Clusterf32) -> () {
        // let (left_id, right_id) = {
        //     if let Some([left, right]) = node.children() {
        //         (left.name(), right.name())
        //     } else {
        //         ("default".to_string(), "default".to_string())
        //     }
        // };
        // if self.id.is_empty() {
        //     debug!("warning unity struct was sent with null id");
        //     self.id = StringFFI::new(node.name());
        // }

        // if self.left_id.is_empty() {
        //     debug!("warning unity struct was sent with null lid");

        //     self.left_id = StringFFI::new(left_id);
        // }

        // if self.right_id.is_empty() {
        //     debug!("warning unity struct was sent with null rid");

        //     self.right_id = StringFFI::new(right_id);
        // }

        self.cardinality = node.cardinality() as i32;
        self.depth = node.depth() as i32;
        self.radius = node.radius() as f32;
        self.lfd = node.lfd() as f32;
        self.arg_center = node.arg_center() as i32;
        self.arg_radius = node.arg_radius() as i32;
    }

    pub fn from_clam(node: &Clusterf32) -> Self {
        let (left_id, right_id) = {
            if let Some([left, right]) = node.children() {
                (left.name(), right.name())
            } else {
                ("None".to_string(), "None".to_string())
            }
        };

        ClusterData {
            pos: glam::Vec3::new(0., 0., 0.),
            color: glam::Vec3::new(0., 0., 0.),
            id: (StringFFI::new(node.name())),
            left_id: StringFFI::new(left_id),
            right_id: StringFFI::new(right_id),
            cardinality: (node.cardinality() as i32),
            depth: (node.depth() as i32),
            radius: node.radius(),
            lfd: node.lfd() as f32,
            arg_center: (node.arg_center() as i32),
            arg_radius: (node.arg_radius() as i32),
            dist_to_query: -1f32,
        }
    }

    pub fn from_reingold_node(other: &reingold_impl::Node) -> Self {
        let (left, right) = other.get_child_names();
        ClusterData {
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
            radius: -1.0,
            lfd: -1.0,
            arg_center: -1,
            arg_radius: -1,
            dist_to_query: -1f32,
        }
    }
    pub fn free_ids(&mut self) {
        self.id.free();
        self.left_id.free();
        self.right_id.free();
    }
}
