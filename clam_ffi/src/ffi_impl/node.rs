#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    ffi::{c_char, CString},
    ptr::{null, null_mut},
};

use super::reingold_impl::{self, Node};

extern crate nalgebra as na;

pub struct NodeBaton {
    x: f32,
    y: f32,
    z: f32,
    r: f32,
    g: f32,
    b: f32,
    // id: *mut i8,
    // left: *mut i8,
    // right: *mut i8,
}

impl NodeBaton {
    pub fn new_from_reingold_node(node: &reingold_impl::Node) -> Self {
        let (left_name, right_name) = {
            if !node.is_leaf() {
                (
                    node.get_left_child().unwrap().as_ref().borrow().get_name(),
                    node.get_right_child().unwrap().as_ref().borrow().get_name(),
                )
            } else {
                (String::new(), String::new())
            }
        };
        let color = node.get_color();

        NodeBaton {
            x: node.get_x(),
            y: node.get_y(),
            z: 0.,
            r: color.x,
            g: color.y,
            b: color.z,
            // id: null_mut() as *mut i8,
            // left: null_mut() as *mut i8,
            // right: null_mut() as *mut i8,
            // id: CString::new(node.get_name()).expect("").into_raw(),
            // left: CString::new(left_name).expect("").into_raw(),
            // right: CString::new(right_name).expect("").into_raw(),
        }

        // NodeBaton {
        //     x: node.pos.x,
        //     y: node.pos.y,
        //     z: node.pos.z,
        //     r: node.color.x,
        //     g: node.color.y,
        //     b: node.color.z,
        //     id: CString::new(node.id.clone()).expect("").into_raw(),
        //     left: CString::new(node.id.clone()).expect("").into_raw(),
        //     right: CString::new(node.id.clone()).expect("").into_raw(),
        // }
    }

    // pub fn set_ids(&mut self, id: *mut i8, left: *mut i8, right: *mut i8) {
    //     self.id = id;
    //     self.left = left;
    //     self.right = right;
    // }

    pub fn to_ptr(self) -> *mut NodeBaton {
        Box::into_raw(Box::new(self))
    }

    pub fn free(ptr: *mut NodeBaton) {
        if ptr.is_null() {
            return;
        }
        unsafe {
            let b = Box::from_raw(ptr);
            // let test = CString::from_raw(b.id as *mut c_char);
            // let test2 = CString::from_raw(b.left as *mut c_char);
            // let test3 = CString::from_raw(b.right as *mut c_char);
        }
    }
}

pub struct NodeI {
    pos: na::Vector3<f32>,
    color: na::Vector3<f32>,
    id: String,
    left_child: String,
    right_child: String,
}

impl NodeI {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        r: f32,
        g: f32,
        b: f32,
        id: String,
        left: String,
        right: String,
    ) -> Self {
        NodeI {
            pos: na::Vector3::new(x, y, z),
            color: (na::Vector3::new(r, g, b)),
            id: id,
            left_child: left,
            right_child: right,
        }
    }

    // pub unsafe fn from_baton(baton: NodeBaton) -> Self {
    //     return Self::new(
    //         baton.x,
    //         baton.y,
    //         baton.z,
    //         baton.r,
    //         baton.g,
    //         baton.b,
    //         String::from(CString::from_raw(baton.id).to_str()),
    //         baton.left,
    //         baton.right,
    //     );
    // }

    pub fn get_pos(&self) -> &na::Vector3<f32> {
        &self.pos
    }

    pub fn get_color(&self) -> &na::Vector3<f32> {
        &self.color
    }

    pub fn get_left_child(&self) -> &String {
        &self.left_child
    }

    pub fn get_right_child(&self) -> &String {
        &self.right_child
    }
    pub fn get_id(&self) -> &String {
        &self.id
    }
}
