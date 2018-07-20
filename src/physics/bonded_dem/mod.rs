#[macro_use]
pub mod equations;
mod tests;

// local imports
use contact_search::{NNPSMutParts, NNPS};
use std::collections::HashMap;

// external crate imports
use cm::{Vector3};

#[derive(Clone, Debug)]
pub struct Bond {
    tang_overlap: Vector3<f32>,
}

impl Bond {
    pub fn new() -> Self {
        Bond {
            tang_overlap: Vector3::new(0., 0., 0.),
        }
    }
}
pub struct DemBonded {
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
    pub bonds: Vec<HashMap<usize, Bond>>,
    pub bonds0: Vec<HashMap<usize, Bond>>,
}

impl DemBonded {
    pub fn new(len: usize, id: usize, name: String) -> Self {
        DemBonded {
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
            bonds: vec![HashMap::new(); len],
            bonds0: vec![HashMap::new(); len],
        }
    }
}

pub struct DemBondedDstStrkt<'a> {
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
    pub bonds: &'a mut Vec<HashMap<usize, Bond>>,
    pub bonds0: &'a mut Vec<HashMap<usize, Bond>>,
}

pub struct DemBondedSrcStrkt<'a> {
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

pub trait DemBondedDstTrait :NNPS{
    fn get_parts_mut(&mut self) -> DemBondedDstStrkt;
}

pub trait DemBondedSrcTrait : NNPS{
    fn get_parts_mut(&mut self) -> DemBondedSrcStrkt;
}

#[macro_export]
macro_rules! impl_DemBondedDstTrait{
    ($($t:ty)*) => ($(
        impl DemBondedDstTrait for $t {
            fn get_parts_mut(&mut self) -> DemBondedDstStrkt {
                DemBondedDstStrkt{
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
                    bonds: &mut self.bonds,
                    bonds0: &mut self.bonds0,
                }
            }
        }
    )*)
}

#[macro_export]
macro_rules! impl_DemBondedSrcTrait{
    ($($t:ty)*) => ($(
        impl DemBondedSrcTrait for $t {
            fn get_parts_mut(&mut self) -> DemBondedSrcStrkt {
                DemBondedSrcStrkt{
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

impl_nnps![DemBonded];
impl_DemBondedDstTrait![DemBonded];
impl_DemBondedSrcTrait![DemBonded];
