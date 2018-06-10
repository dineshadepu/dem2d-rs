#![allow(dead_code)]
#![allow(unused_imports)]
#[macro_use]
extern crate ndarray;
use ndarray::prelude::*;

// local modules
pub mod base;
pub mod contact_search;
pub mod dem;
pub mod geometry;
pub mod integrator;

/// An entity in rudem. It's props are basic and every other particle entity
/// must have to have these attributes for neighbour search dumping output and
/// other functionalities.  Each and every type of struct has to implement a
/// trait `Base` for the basic library functionalities

pub struct DemDiscrete {
    pub len: usize,
    pub m: Array1<f32>,
    pub x: Array1<f32>,
    pub y: Array1<f32>,
    pub u: Array1<f32>,
    pub v: Array1<f32>,
    pub omega: Array1<f32>,
    pub inertia: Array1<f32>,
    pub h: Array1<f32>,
    pub m_inv: Array1<f32>,
    pub rad: Array1<f32>,
    pub fx: Array1<f32>,
    pub fy: Array1<f32>,
    pub id: usize,
}

impl DemDiscrete {
    pub fn new(len: usize, id: usize) -> Self {
        DemDiscrete {
            len,
            m: Array1::zeros(len),
            x: Array1::zeros(len),
            y: Array1::zeros(len),
            u: Array1::zeros(len),
            v: Array1::zeros(len),
            omega: Array1::zeros(len),
            inertia: Array1::zeros(len),
            h: Array1::zeros(len),
            m_inv: Array1::zeros(len),
            rad: Array1::zeros(len),
            fx: Array1::zeros(len),
            fy: Array1::zeros(len),
            id: id,
        }
    }
    pub fn new_x(x: Array1<f32>, id: usize) -> Self {
        let len = x.len();
        DemDiscrete {
            len,
            x,
            id,
            m: Array1::zeros(len),
            y: Array1::zeros(len),
            u: Array1::zeros(len),
            v: Array1::zeros(len),
            omega: Array1::zeros(len),
            inertia: Array1::zeros(len),
            h: Array1::zeros(len),
            m_inv: Array1::zeros(len),
            rad: Array1::zeros(len),
            fx: Array1::zeros(len),
            fy: Array1::zeros(len),
        }
    }
    pub fn new_x_y(x: Array1<f32>, y: Array1<f32>, id: usize) -> Self {
        let len = x.len();
        DemDiscrete {
            len,
            x,
            y,
            id,
            m: Array1::zeros(len),
            u: Array1::zeros(len),
            v: Array1::zeros(len),
            omega: Array1::zeros(len),
            inertia: Array1::zeros(len),
            h: Array1::zeros(len),
            m_inv: Array1::zeros(len),
            rad: Array1::zeros(len),
            fx: Array1::zeros(len),
            fy: Array1::zeros(len),
        }
    }

    pub fn new_x_y_h(x: Array1<f32>, y: Array1<f32>, h: Array1<f32>, id: usize) -> Self {
        let len = x.len();
        DemDiscrete {
            len,
            x,
            y,
            id,
            m: Array1::zeros(len),
            u: Array1::zeros(len),
            v: Array1::zeros(len),
            omega: Array1::zeros(len),
            inertia: Array1::zeros(len),
            h,
            m_inv: Array1::zeros(len),
            rad: Array1::zeros(len),
            fx: Array1::zeros(len),
            fy: Array1::zeros(len),
        }
    }
}
