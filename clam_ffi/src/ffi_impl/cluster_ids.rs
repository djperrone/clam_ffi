use crate::{debug, utils::types::Clusterf32};

use super::string_ffi::StringFFI;

pub struct ClusterIDsWrapper {
    data: ClusterIDs,
}

impl Drop for ClusterIDsWrapper {
    fn drop(&mut self) {
        // debug!("Freeing Cluster Data string with rust destructor");
        self.data.free_ids();
    }
}

impl ClusterIDsWrapper {
    pub fn from_cluster(cluster: &Clusterf32) -> Self {
        ClusterIDsWrapper {
            data: ClusterIDs::from_clam(cluster),
        }
    }

    pub fn data(&self) -> &ClusterIDs {
        &self.data
    }
    pub fn data_mut(&mut self) -> &mut ClusterIDs {
        &mut self.data
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ClusterIDs {
    pub id: StringFFI,
    pub left_id: StringFFI,
    pub right_id: StringFFI,
    // pub message: StringFFI,
}

impl ClusterIDs {
    pub unsafe fn get_id(&self) -> String {
        self.id.as_string().unwrap()
    }

    pub unsafe fn get_ffi_id(&self) -> &StringFFI {
        &self.id
    }

    pub fn from_clam(node: &Clusterf32) -> Self {
        let (left_id, right_id) = {
            if let Some([left, right]) = node.children() {
                (left.name(), right.name())
            } else {
                ("None".to_string(), "None".to_string())
            }
        };

        ClusterIDs {
            id: (StringFFI::new(node.name())),
            left_id: StringFFI::new(left_id),
            right_id: StringFFI::new(right_id),
            // message: StringFFI::new(std::iter::repeat(' ').take(50).collect()),
        }
    }

    pub fn free_ids(&mut self) {
        self.id.free();
        self.left_id.free();
        self.right_id.free();
        // self.message.free();
    }
}
