#![allow(dead_code)]
#![allow(unused_variables)]

extern crate nalgebra as na;

pub struct NodeBaton {
    x: f32,
    y: f32,
    z: f32,
    r: f32,
    g: f32,
    b: f32,
    id: String,
    left: String,
    right: String,
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

    pub fn from_baton(baton: NodeBaton) -> Self {
        return Self::new(
            baton.x,
            baton.y,
            baton.z,
            baton.r,
            baton.g,
            baton.b,
            baton.id,
            baton.left,
            baton.right,
        );
    }

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
