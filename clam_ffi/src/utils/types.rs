use abd_clam::{core::cluster::Cluster, core::dataset::VecDataset, Cakes};

use crate::handle::handle::Handle;

pub type OutHandlePtr<'a> = Option<&'a mut *mut Handle>;

pub type InHandlePtr<'a> = Option<&'a mut Handle>;

pub type Clusterf32 = Cluster<f32>;
pub type DataSet = VecDataset<Vec<f32>, f32>;
pub type Cakesf32 = Cakes<f32, f32, DataSet>;
