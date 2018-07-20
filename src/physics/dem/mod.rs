#[macro_use]
pub mod equations;

// local imports
use contact_search::{NNPSMutParts, NNPS};
use std::collections::HashMap;

// external crate imports
use cm::{Vector3, Zero};
use ndarray::prelude::*;

pub struct DemDiscrete {
    pub len: usize,
    pub m: Vec<f32>,
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub u: Vec<f32>,
    pub v: Vec<f32>,
    pub omega_z: Vec<f32>,
    pub x0: Vec<f32>,
    pub y0: Vec<f32>,
    pub u0: Vec<f32>,
    pub v0: Vec<f32>,
    pub omega_z0: Vec<f32>,
    pub inertia: Vec<f32>,
    pub h: Vec<f32>,
    pub m_inv: Vec<f32>,
    pub i_inv: Vec<f32>,
    pub rad: Vec<f32>,
    pub fx: Vec<f32>,
    pub fy: Vec<f32>,
    pub tauz: Vec<f32>,
    pub id: usize,
    pub name: String,
    pub tang_history: Vec<HashMap<usize, HashMap<usize, Vector3<f32>>>>,
    pub tang_history0: Vec<HashMap<usize, HashMap<usize, Vector3<f32>>>>,
}

impl DemDiscrete {
    pub fn new(len: usize, id: usize, name: String) -> Self {
        DemDiscrete {
            len,
            name,
            id,
            m: vec![0.; len],
            x: vec![0.; len],
            y: vec![0.; len],
            u: vec![0.; len],
            v: vec![0.; len],
            omega_z: vec![0.; len],
            x0: vec![0.; len],
            y0: vec![0.; len],
            u0: vec![0.; len],
            v0: vec![0.; len],
            omega_z0: vec![0.; len],
            inertia: vec![0.; len],
            h: vec![0.; len],
            m_inv: vec![0.; len],
            i_inv: vec![0.; len],
            rad: vec![0.; len],
            fx: vec![0.; len],
            fy: vec![0.; len],
            tauz: vec![0.; len],
            tang_history: vec![HashMap::new(); len],
            tang_history0: vec![HashMap::new(); len],
        }
    }
}

pub struct DemDiscreteDstStrkt<'a> {
    pub len: &'a mut usize,
    pub m: &'a mut Vec<f32>,
    pub x: &'a mut Vec<f32>,
    pub y: &'a mut Vec<f32>,
    pub u: &'a mut Vec<f32>,
    pub v: &'a mut Vec<f32>,
    pub omega_z: &'a mut Vec<f32>,
    pub inertia: &'a mut Vec<f32>,
    pub h: &'a mut Vec<f32>,
    pub m_inv: &'a mut Vec<f32>,
    pub i_inv: &'a mut Vec<f32>,
    pub rad: &'a mut Vec<f32>,
    pub fx: &'a mut Vec<f32>,
    pub fy: &'a mut Vec<f32>,
    pub tauz: &'a mut Vec<f32>,
    pub id: &'a mut usize,
    pub name: &'a mut String,
    pub tang_history: &'a mut Vec<HashMap<usize, HashMap<usize, Vector3<f32>>>>,
    pub tang_history0: &'a mut Vec<HashMap<usize, HashMap<usize, Vector3<f32>>>>,
}

pub struct DemDiscreteSrcStrkt<'a> {
    pub m: &'a mut Vec<f32>,
    pub x: &'a mut Vec<f32>,
    pub y: &'a mut Vec<f32>,
    pub u: &'a mut Vec<f32>,
    pub v: &'a mut Vec<f32>,
    pub omega_z: &'a mut Vec<f32>,
    pub inertia: &'a mut Vec<f32>,
    pub h: &'a mut Vec<f32>,
    pub m_inv: &'a mut Vec<f32>,
    pub i_inv: &'a mut Vec<f32>,
    pub rad: &'a mut Vec<f32>,
    pub id: &'a mut usize,
    pub name: &'a mut String,
}

pub trait DemDiscreteDstTrait: NNPS {
    fn get_parts_mut(&mut self) -> DemDiscreteDstStrkt;
}

pub trait DemDiscreteSrcTrait: NNPS {
    fn get_parts_mut(&mut self) -> DemDiscreteSrcStrkt;
}

#[macro_export]
macro_rules! impl_DemDiscreteDstTrait{
    ($($t:ty)*) => ($(
        impl DemDiscreteDstTrait for $t {
            fn get_parts_mut(&mut self) -> DemDiscreteDstStrkt {
                DemDiscreteDstStrkt{
                    len: &mut self.len,
                    m: &mut self.m,
                    x: &mut self.x,
                    y: &mut self.y,
                    u: &mut self.u,
                    v: &mut self.v,
                    omega_z: &mut self.omega_z,
                    inertia: &mut self.inertia,
                    h: &mut self.h,
                    m_inv: &mut self.m_inv,
                    i_inv: &mut self.i_inv,
                    rad: &mut self.rad,
                    fx: &mut self.fx,
                    fy: &mut self.fy,
                    tauz: &mut self.tauz,
                    id: &mut self.id,
                    name: &mut self.name,
                    tang_history: &mut self.tang_history,
                    tang_history0: &mut self.tang_history0,
                }
            }
        }
    )*)
}

#[macro_export]
macro_rules! impl_DemDiscreteSrcTrait{
    ($($t:ty)*) => ($(
        impl DemDiscreteSrcTrait for $t {
            fn get_parts_mut(&mut self) -> DemDiscreteSrcStrkt {
                DemDiscreteSrcStrkt{
                    m: &mut self.m,
                    x: &mut self.x,
                    y: &mut self.y,
                    u: &mut self.u,
                    v: &mut self.v,
                    omega_z: &mut self.omega_z,
                    inertia: &mut self.inertia,
                    h: &mut self.h,
                    m_inv: &mut self.m_inv,
                    i_inv: &mut self.i_inv,
                    rad: &mut self.rad,
                    id: &mut self.id,
                    name: &mut self.name,
                }
            }
        }
    )*)
}

impl_nnps![DemDiscrete];
impl_DemDiscreteDstTrait![DemDiscrete];
impl_DemDiscreteSrcTrait![DemDiscrete];
