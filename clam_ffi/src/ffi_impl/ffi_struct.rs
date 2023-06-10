// // vek::vec::repr_c

// // // take awat reprc
// // // create struct Nodeforc#

// // #[repr(C)]
// // pub struct NodeBaton {
// //     x: f32,
// //     y: f32,
// //     r: f32,
// //     g: f32,
// //     b: f32,
// //     id: i32,
// //     left: i32,
// //     right: i32,
// // }

// // #[repr(C)]
// // pub struct NodeI {
// //     pos: Vec2,
// //     color: Vec3,
// //     id: i32,
// //     left_child: i32,
// //     right_child: i32,
// // }

// // impl<'a> NodeI {
// //     pub fn new(
// //         x: f32,
// //         y: f32,
// //         r: f32,
// //         g: f32,
// //         b: f32,
// //         id: i32,
// //         left_child: i32,
// //         right_child: i32,
// //     ) -> Self {
// //         NodeI {
// //             pos: Vec2::new(x, y),
// //             color: Vec3::new(r, g, b),
// //             id: id,
// //             left_child: left_child,
// //             right_child: right_child,
// //         }
// //     }

// //     pub fn default() -> Self {
// //         NodeI {
// //             pos: Vec2::new(0., 0.),
// //             color: Vec3::new(0., 0., 0.),
// //             id: -1,
// //             left_child: -1,
// //             right_child: -1,
// //         }
// //     }

// //     // fn to_ptr(self) -> *mut Self {
// //     //   unsafe { transmute(Box::new(self)) }
// //     // }

// //     // fn from_ptr(ptr: *mut NodeI) -> &'a mut Self {
// //     //     unsafe { &mut *ptr }
// //     // }

// //     pub fn get_pos(&self) -> &Vec2 {
// //         &self.pos
// //     }

// //     pub fn get_color(&self) -> &Vec3 {
// //         &self.color
// //     }

// //     pub fn get_left_child(&self) -> i32 {
// //         self.left_child
// //     }

// //     pub fn get_right_child(&self) -> i32 {
// //         self.right_child
// //     }
// //     pub fn get_id(&self) -> i32 {
// //         self.id
// //     }
// // }

// // #[derive(Clone)]
// // #[repr(C)]
// // pub struct Vec2 {
// //     x: f32,
// //     y: f32,
// // }

// // impl<'a> Vec2 {
// //     pub fn new(x: f32, y: f32) -> Self {
// //         Vec2 { x: x, y: y }
// //     }

// //     pub fn get(&self) -> (f32, f32) {
// //         (self.x, self.y)
// //     }

// //     pub fn get_x(&self) -> f32 {
// //         self.x
// //     }
// //     pub fn get_y(&self) -> f32 {
// //         self.y
// //     }
// // }

// #[derive(Clone)]
// #[repr(C)]
// pub struct Vec3 {
//     x: f32,
//     y: f32,
//     z: f32,
// }

// impl<'a> Vec3 {
//     pub fn default() -> Vec3 {
//         Vec3 {
//             x: 0.,
//             y: 0.,
//             z: 0.,
//         }
//     }
//     pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
//         Vec3 { x: x, y: y, z: z }
//     }

//     pub fn get(&self) -> (f32, f32, f32) {
//         (self.x, self.y, self.z)
//     }

//     pub fn get_x(&self) -> f32 {
//         self.x
//     }
//     pub fn get_y(&self) -> f32 {
//         self.y
//     }
//     pub fn get_z(&self) -> f32 {
//         self.z
//     }
// }
