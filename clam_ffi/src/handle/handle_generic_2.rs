// // // use abd_clam::{Cakes, Dataset, Instance, PartitionCriteria, VecDataset};
// // // use distances::Number;
//
// // // use crate::utils::distances::levenshtein;
//
// // // pub struct HandleGeneric<I: Instance, U: Number, D: Dataset<I, U>> {
// // //     cakes: Option<Cakes<I, U, D>>,
// // // }
//
// // // impl<I: Instance, U: Number, D: Dataset<I, U>> HandleGeneric<I, U, D> {
//
// // //     pub fn from_cakes(cakes: Cakes<I, U, D>) -> Self {
// // //         Self {
// // //             cakes: Some(cakes), // tree: Tree::new(data, seed).partition(criteria),
// // //                                 // best_knn: None,
// // //         }
// // //     }
// // // }
//
// // // fn test_str_handle<I: Instance, U: Number, D: Dataset<I, U>>(
// // //     ptr: Option<&mut *mut HandleGeneric<I, U, D>>,
// // // ) {
// // //     if let Some(out_handle) = ptr {
// // //         let seed = 42;
// // //         let seq_len = 100;
//
// // //         let data =
// // //             symagen::random_data::random_string(100, seq_len, seq_len, "abcdefghijklmnop", seed);
// // //         let data = VecDataset::new("test".to_string(), data.clone(), levenshtein::<u16>, false);
// // //         let cakes = Cakes::new(data, Some(42), &PartitionCriteria::default());
//
// // //         let handle = HandleGeneric::from_cakes(cakes);
// // //         *out_handle = Box::into_raw(Box::new(handle));
// // //     }
// // // }
//
// // #[repr(C)]
// // pub struct HandleGeneric {
// //     float_data: Option<f32>,
// //     int_data: Option<i32>,
// // }
// //
// // #[no_mangle]
// // pub extern "C" fn create_handle_float(ptr: Option<&mut *mut HandleGeneric>, value: f32) {
// //     if let Some(out_handle) = ptr {
// //         let handle = HandleGeneric {
// //             float_data: Some(value),
// //             int_data: None,
// //         };
// //         *out_handle = Box::into_raw(Box::new(handle));
// //     }
// // }
// //
// // #[no_mangle]
// // pub extern "C" fn create_handle_int(ptr: Option<&mut *mut HandleGeneric>, value: i32) {
// //     if let Some(out_handle) = ptr {
// //         let handle = HandleGeneric {
// //             float_data: None,
// //             int_data: Some(value),
// //         };
// //         *out_handle = Box::into_raw(Box::new(handle));
// //     }
// // }
//
// // pub extern "C" fn create_handle(ptr: Option<&mut *mut HandleGeneric<T>>, handle_type: String) {
// //     if handle_type == "f32" {
// //         create_handle_float(ptr, 0.0);
// //     } else if handle_type == "i32" {
// //         create_handle_int(ptr, 0);
// //     } else {
// //         panic!()
// //     }
// // }
//
// // #[repr(C)]
// // pub enum HandleType {
// //     Int,
// //     Float,
// // }
// //
// // #[repr(C)]
// // pub struct HandleGeneric<T> {
// //     data: T,
// // }
// //
// // #[no_mangle]
// // pub extern "C" fn create_handle_int(ptr: Option<&mut *mut HandleGeneric<i32>>, value: i32) {
// //     if let Some(out_handle) = ptr {
// //         let handle = HandleGeneric { data: value };
// //         *out_handle = Box::into_raw(Box::new(handle));
// //     }
// // }
// //
// // #[no_mangle]
// // pub extern "C" fn create_handle_float(ptr: Option<&mut *mut HandleGeneric<f32>>, value: f32) {
// //     if let Some(out_handle) = ptr {
// //         let handle = HandleGeneric { data: value };
// //         *out_handle = Box::into_raw(Box::new(handle));
// //     }
// // }
// //
// // use std::mem;
// //
// // #[no_mangle]
// // pub extern "C" fn create_handle(
// //     ptr: Option<&mut *mut HandleGeneric<std::ffi::c_void>>,
// //     handle_type: HandleType,
// // ) {
// //     match handle_type {
// //         HandleType::Int => {
// //             let int_ptr  = ptr.map(|p| transmute_ptr(p));
// //             create_handle_int(int_ptr, 0);
// //         }
// //         HandleType::Float => {
// //             let float_ptr = ptr.map(|p| transmute_ptr(p));
// //             create_handle_float(float_ptr, 0.0);
// //         }
// //     }
// // }
// //
// // // Helper function to safely transmute between pointer types
// // fn transmute_ptr<T, U>(ptr: *mut T) -> *mut U {
// //     unsafe { mem::transmute(ptr) }
// // }
//
// pub struct HandleGeneric<T,U,I> {
//     data: T,
//     data1: U,
//     data2: I,
// }
//
// impl<T,U,I> HandleGeneric<T,U,I> {
//     #[no_mangle]
//     pub extern "C" fn create_handle_int(ptr: Option<&mut *mut HandleGeneric<T,U,I>>, value: i32, value1: f32, value2: u8) {
//         if let Some(out_handle) = ptr {
//             let handle = HandleGeneric { data: value,data1 : value2, data2: value2};
//             *out_handle = Box::into_raw(Box::new(handle));
//         }
//     }
// }
