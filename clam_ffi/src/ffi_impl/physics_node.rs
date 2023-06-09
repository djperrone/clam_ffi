use crate::CBFnNodeVisitor;

use super::{handle::Clusterf32, node::NodeData, physics};

pub struct PhysicsNode {
    position: glam::Vec3,
    friction: f32,
    max_speed: f32,
    velocity: glam::Vec3,
    acceleration: glam::Vec3,
    // sphere: kiss3d::scene::SceneNode,
    // point: kiss3d::nalgebra::Point3<f32>,
    // cluster: &'a Clusterf32,
    mass: f32,
}

impl PhysicsNode {
    pub fn new(node_data: &NodeData, cluster: &Clusterf32) -> Self {
        PhysicsNode {
            position: node_data.pos,
            friction: 0.98,
            max_speed: 5.,
            velocity: glam::Vec3::new(0., 0., 0.),
            acceleration: glam::Vec3::new(0., 0., 0.),
            mass: cluster.cardinality() as f32,
            // cluster: cluster,
        }
    }

    pub fn mass(&self) -> f32 {
        self.mass
    }

    // pub fn cluster(&self) -> &Clusterf32 {
    //     self.cluster
    // }

    // F = M * A
    //updates acceleration of node
    pub fn accelerate(&mut self, force: glam::Vec3) {
        // A = F / M
        self.acceleration = self.acceleration + (force / self.mass());
    }

    pub fn get_position(&self) -> glam::Vec3 {
        self.position
    }

    //applies acceleration to velocity, applies velocity of node's position then updates sphere object on canvas
    pub fn update_position(&mut self) {
        self.velocity += self.acceleration;
        self.velocity *= self.friction; //reduce velocity by applying friction

        //if current velocity > max_speed, set velocity to max speed (to prevent extreme rubber banding in some graphs)
        if physics::get_magnitude(self.velocity) > self.max_speed {
            self.velocity = physics::set_magnitude(self.velocity, self.max_speed);
        }

        //sets back to origin (look further into kiss3d so you dont have to use translations)
        // self.sphere.append_translation(&Translation3::new(-self.position.x, -self.position.y, -self.position.z));

        self.position += self.velocity;

        //resets accel
        self.acceleration.x = 0.;
        self.acceleration.y = 0.;
        self.acceleration.z = 0.;

        //     let ffi_data = NodeData::new()
        // node_visitor()
        //appends translation to sphere canvas object at the x,y,z (from origin)
        // self.sphere.append_translation(&Translation3::new(self.position.x, self.position.y, self.position.z));

        //updates position of point object (kiss3d draws lines between point objects so these are how visible edges are drawn in graph)
        // self.point.x = self.position[0];
        // self.point.y = self.position[1];
        // self.point.z = self.position[2];
    }
}
