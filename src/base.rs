use DemDiscrete;
use ndarray::Array1;

pub struct BasePartsMut<'a> {
    pub len: &'a mut usize,
    pub m: &'a mut Array1<f32>,
    pub x: &'a mut Array1<f32>,
    pub y: &'a mut Array1<f32>,
    pub u: &'a mut Array1<f32>,
    pub v: &'a mut Array1<f32>,
    pub omega: &'a mut Array1<f32>,
    pub inertia: &'a mut Array1<f32>,
    pub h: &'a mut Array1<f32>,
    pub m_inv: &'a mut Array1<f32>,
    pub rad: &'a mut Array1<f32>,
    pub fx: &'a mut Array1<f32>,
    pub fy: &'a mut Array1<f32>,
    pub id: &'a mut usize,
}

pub trait Base {
    fn get_parts_mut(&mut self) -> BasePartsMut;
    fn get_x(&self) -> &Array1<f32>;
    fn get_y(&self) -> &Array1<f32>;
}

impl Base for DemDiscrete {
    fn get_parts_mut(&mut self) -> BasePartsMut {
        BasePartsMut {
            len: &mut self.len,
            m: &mut self.m,
            x: &mut self.x,
            y: &mut self.y,
            u: &mut self.u,
            v: &mut self.v,
            omega: &mut self.omega,
            inertia: &mut self.inertia,
            h: &mut self.h,
            m_inv: &mut self.m_inv,
            rad: &mut self.rad,
            fx: &mut self.fx,
            fy: &mut self.fy,
            id: &mut self.id,
        }
    }

    fn get_x(&self) -> &Array1<f32> {
        &self.x
    }
    fn get_y(&self) -> &Array1<f32> {
        &self.y
    }
}
