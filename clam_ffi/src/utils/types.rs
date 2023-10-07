use abd_clam::{cluster::Cluster, dataset::VecVec, search::cakes::CAKES};

use crate::handle::handle::Handle;

pub type OutHandlePtr<'a> = Option<&'a mut *mut Handle>;

pub type InHandlePtr<'a> = Option<&'a mut Handle>;

pub type Clusterf32 = Cluster<f32, f32, VecVec<f32, f32>>;
pub type DataSet = VecVec<f32, f32>;
pub type Cakesf32 = CAKES<f32, f32, VecVec<f32, f32>>;
