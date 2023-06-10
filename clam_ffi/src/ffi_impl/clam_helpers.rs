use clam::utils::helpers;

use crate::debug;

use super::{handle::Handle, node::NodeFFI};

// #[no_mangle]
// // #[ffi_function(xx = "123", tags(csharp::a = "csharp1",))]
// pub unsafe extern "C" fn get_node_data(
//     context: Option<&mut Handle>,
//     name: *const u8,
//     name_len: i32,
//     incoming: Option<&NodeFFI>,
//     outgoing: Option<&mut NodeFFI>,
// ) -> () {
//     debug!("tyest1");
//     match context {
//         Some(handle) => {
//             debug!("tyest2");

//             let node_name = crate::helpers::csharp_to_rust_utf8(name, name_len);
//             debug!("tyest3");

//             if let Some(in_node) = incoming {
//                 debug!("tyest4");

//                 if let Some(out_node) = outgoing {
//                     debug!("tyest5");

//                     *out_node = *in_node;
//                     debug!("tyest6");

//                     match node_name {
//                         Ok(node_name) => {
//                             debug!("tyest7");
//                             let node = handle.get_node_data(node_name.chars().rev().collect());
//                             debug!("tyest8");

//                             if let Ok(data) = node {
//                                 debug!("tyest9");

//                                 out_node.from_copy(&data);
//                                 debug!("tyest10");

//                             }
//                         }
//                         Err(e) => {
//                             debug!("tyest11");

//                             debug!("{}", e);
//                         }
//                     }
//                 }
//             }
//         }
//         None => debug!("get node data handle not found"),
//     }
//     // let incoming = incoming.unwrap();
//     // let outgoing = outgoing.unwrap();

//     // *outgoing = *incoming;
//     // outgoing.set_data_from_clam();

//     // outgoing.ammo *= 2;
//     // outgoing.player_1.x *= 2.0;
//     // outgoing.player_1.y *= 2.0;
//     // outgoing.player_1.z *= 2.0;
//     // outgoing.player_2.x *= 2.0;
//     // outgoing.player_2.y *= 2.0;
//     // outgoing.player_2.z *= 2.0;
// }
