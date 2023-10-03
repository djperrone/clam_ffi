// #![allow(dead_code)]
// #![allow(unused_variables)]

// use std::ffi::{c_char, CStr};

// use crate::ffi_impl::handle::Handle;
// use crate::{debug, CBFnNodeVistor};
// use crate::{
//     ffi_impl::node::{NodeData, StringFFI},
//     utils::helpers,
// };

// #[repr(C)]
// #[derive(Copy, Clone, Debug)]
// pub struct ComplexStruct {
//     my_str: StringStruct1,
// }

// #[repr(C)]
// #[derive(Copy, Clone, Debug)]
// pub struct StringStruct1 {
//     pub utf8_str: *mut u8,
//     pub utf8_len: i32,
// }

// impl StringStruct1 {
//     pub fn new(data: String) -> Self {
//         StringStruct1 {
//             utf8_str: helpers::alloc_to_c_char(data.clone()) as *mut u8,
//             utf8_len: data.len() as i32,
//         }
//     }
// }

// #[repr(C)]
// #[derive(Clone, Debug)]
// pub struct StringStruct2 {
//     pub s: String,
// }

// impl StringStruct2 {
//     pub unsafe fn new(other: &StringStruct1) -> Self {
//         StringStruct2 {
//             s: helpers::csharp_to_rust_utf8(other.utf8_str, other.utf8_len)
//                 .unwrap_or("failed to do stuff".to_string()),
//         }
//     }
// }

// #[no_mangle]
// pub unsafe extern "C" fn test_node_rust_alloc(
//     incoming: Option<&NodeData>,
//     outgoing: Option<&mut NodeData>,
// ) {
//     // let mystr = helpers::alloc_to_c_char("hello123".to_string());
//     // let ffi_string = StringStruct1::new("hello123".to_string());
//     if let Some(in_data) = incoming {
//         if let Some(out_data) = outgoing {
//             *out_data = *in_data;
//             out_data.id = StringFFI::new("hello123".to_string());
//             // out_data.my_str.utf8_str = helpers::alloc_to_c_char("hello123".to_string()) as *mut u8;

//             debug!("string struct test 123 {:?}", out_data.id.as_string());
//             // helpers::free_c_char(out_data.my_str.utf8_str as *mut i8);
//             // std::mem::forget(out_data.my_str);
//         }
//     }
// }

// #[no_mangle]
// pub unsafe extern "C" fn test_string_struct_complex(
//     incoming: Option<&NodeData>,
//     outgoing: Option<&mut NodeData>,
// ) {
//     // let mystr = helpers::alloc_to_c_char("hello123".to_string());
//     // let ffi_string = StringStruct1::new("hello123".to_string());
//     if let Some(in_data) = incoming {
//         if let Some(out_data) = outgoing {
//             *out_data = *in_data;
//             // out_data.id = StringFFI::new("hello123".to_string());
//             // out_data.my_str.utf8_str = helpers::alloc_to_c_char("hello123".to_string()) as *mut u8;
//             let some_str = helpers::csharp_to_rust_utf8(in_data.id.data, in_data.id.len);
//             debug!("string struct test 123 {:?}", *out_data.id.data);
//             debug!("string struct test 1234 {:?}", out_data.id.as_string());
//             debug!("string struct test 123 x {:?}", out_data.pos.x);
//             debug!("string struct test 123 str {:?}", some_str);
//             // helpers::free_c_char(out_data.my_str.utf8_str as *mut i8);
//             // std::mem::forget(out_data.my_str);
//         }
//     }
// }

// #[no_mangle]
// pub unsafe extern "C" fn test_string_struct2(
//     incoming: Option<&ComplexStruct>,
//     outgoing: Option<&mut ComplexStruct>,
// ) -> () {
//     if let Some(in_struct) = incoming {
//         // let some_str =
//         //     helpers::csharp_to_rust_utf8(in_struct.my_str.utf8_str, in_struct.my_str.utf8_len);

//         // debug!("start string struct test ");

//         // let ss = StringStruct2::new(in_struct);
//         let tests = "test".to_string();
//         let test = *in_struct.my_str.utf8_str;
//         *(in_struct.my_str.utf8_str.add(1)) = 107;
//         let mut i = 0 as usize;
//         for ch in tests.chars() {
//             *(in_struct.my_str.utf8_str.add(i as usize)) = ch as u8;
//             i += 1;
//         }
//         // debug!("string struct test {:?}", some_str);
//         debug!("string struct test {:?}", *in_struct.my_str.utf8_str)
//     }
// }

// #[no_mangle]
// pub unsafe extern "C" fn test_node_rust_alloc2(
//     context: Option<&mut Handle>,
//     visitor: CBFnNodeVistor,
// ) {
//     // let mystr = helpers::alloc_to_c_char("hello123".to_string());
//     // let ffi_string = StringStruct1::new("hello123".to_string());
//     if let Some(handle) = context {
//         // if let Some(out_data) = outgoing {
//         //     *out_data = *in_data;
//         //     out_data.id = StringFFI::new("hello123".to_string());
//         //     // out_data.my_str.utf8_str = helpers::alloc_to_c_char("hello123".to_string()) as *mut u8;

//         //     debug!("string struct test 123 {:?}", out_data.id.as_string());
//         //     // helpers::free_c_char(out_data.my_str.utf8_str as *mut i8);
//         //     // std::mem::forget(out_data.my_str);
//         // }

//         let data = NodeData::from_clam(&handle.get_root().as_ref().unwrap().as_ref().borrow());
//         visitor(Some(&data));
//     }
// }

// #[no_mangle]
// pub unsafe extern "C" fn test_string_struct_rust_alloc(
//     incoming: Option<&ComplexStruct>,
//     outgoing: Option<&mut ComplexStruct>,
// ) {
//     // let mystr = helpers::alloc_to_c_char("hello123".to_string());
//     // let ffi_string = StringStruct1::new("hello123".to_string());
//     if let Some(in_data) = incoming {
//         if let Some(out_data) = outgoing {
//             *out_data = *in_data;
//             out_data.my_str = StringStruct1::new("hello123".to_string());
//             // out_data.my_str.utf8_str = helpers::alloc_to_c_char("hello123".to_string()) as *mut u8;

//             debug!("string struct test 123 {:?}", out_data.my_str.utf8_str);
//             // helpers::free_c_char(out_data.my_str.utf8_str as *mut i8);
//             // std::mem::forget(out_data.my_str);
//         }
//     }
// }
// #[no_mangle]
// pub extern "C" fn free_string2(
//     incoming: Option<&ComplexStruct>,
//     outgoing: Option<&mut ComplexStruct>,
// ) {
//     debug!("freeing string");
//     if let Some(in_data) = incoming {
//         if let Some(out_data) = outgoing {
//             *out_data = *in_data;
//             // out_data.my_str = StringStruct1::new("hello123".to_string());
//             // out_data.my_str.utf8_str = helpers::alloc_to_c_char("hello123".to_string()) as *mut u8;

//             debug!("string struct test 123 {:?}", out_data.my_str.utf8_str);
//             helpers::free_c_char(out_data.my_str.utf8_str as *mut i8);
//         }
//     }
// }

// #[no_mangle]
// pub unsafe extern "C" fn test_string_struct(
//     incoming: Option<&StringStruct1>,
//     outgoing: Option<&mut StringStruct1>,
// ) -> () {
//     if let Some(in_struct) = incoming {
//         let some_str = helpers::csharp_to_rust_utf8(in_struct.utf8_str, in_struct.utf8_len);

//         // debug!("start string struct test ");

//         // let ss = StringStruct2::new(in_struct);
//         let test = *in_struct.utf8_str;
//         *in_struct.utf8_str = 109;
//         debug!("string struct test {:?}", some_str);
//         debug!("string struct test {:?}", *in_struct.utf8_str)
//     }
// }

// #[no_mangle]
// pub extern "C" fn test_string_fn(s: *const c_char) -> u32 {
//     let c_str = unsafe {
//         assert!(!s.is_null());

//         CStr::from_ptr(s)
//     };
//     debug!("cstr testing {:?}", c_str);
//     let r_str = c_str.to_str().unwrap();
//     r_str.chars().count() as u32
// }

// #[no_mangle]
// pub unsafe extern "C" fn test_struct_array(context: InHandlePtr, arr: *mut NodeData, len: i32) {
//     let test_arr = std::slice::from_raw_parts_mut(arr, len as usize);
//     if let Some(_) = context {
//         if arr.is_null() {
//             return;
//         }
//         let val = *arr;
//         let val1 = test_arr[1];
//         let val2 = test_arr[2];
//         debug!(
//             "array at {}: {}",
//             val.id.as_string().unwrap(),
//             val.cardinality
//         );
//         debug!(
//             "array at {}: {}",
//             val1.id.as_string().unwrap(),
//             val1.cardinality
//         );
//         debug!(
//             "array at {}: {}",
//             val2.id.as_string().unwrap(),
//             val2.cardinality
//         );
//     }
// }
