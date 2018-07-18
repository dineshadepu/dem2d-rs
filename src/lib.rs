#[macro_use]
extern crate itertools;

#[macro_use]
extern crate rulinalg;

#[macro_use]
extern crate cgmath as cm;

#[macro_use]
extern crate ndarray;


// local modules
#[macro_use]
pub mod contact_search;
pub mod geometry;
pub mod integrate;
pub mod math;
pub mod save_data;
pub mod physics;

use contact_search::{NNPS, NNPSMutParts};

use ndarray::prelude::*;
use ndarray::prelude::*;
