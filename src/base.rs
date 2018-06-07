use DemDiscrete;
use ndarray::{Array1};


pub trait Base {
    fn get_m(&self) -> &Array1<f32>;
    fn get_x(&self) -> &Array1<f32>;
    fn get_y(&self) -> &Array1<f32>;
    fn get_u(&self) -> &Array1<f32>;
    fn get_v(&self) -> &Array1<f32>;
    fn get_omega(&self) -> &Array1<f32>;
    fn get_inertia(&self) -> &Array1<f32>;
    fn get_h(&self) -> &Array1<f32>;
    fn get_m_inv(&self) -> &Array1<f32>;
    fn get_rad(&self) -> &Array1<f32>;
    fn get_fx(&self) -> &Array1<f32>;
    fn get_fy(&self) -> &Array1<f32>;
    fn get_id(&self) -> &usize;
    fn get_mut_m(&mut self) -> &mut Array1<f32>;
    fn get_mut_x(&mut self) -> &mut Array1<f32>;
    fn get_mut_y(&mut self) -> &mut Array1<f32>;
    fn get_mut_u(&mut self) -> &mut Array1<f32>;
    fn get_mut_v(&mut self) -> &mut Array1<f32>;
    fn get_mut_omega(&mut self) -> &mut Array1<f32>;
    fn get_mut_inertia(&mut self) -> &mut Array1<f32>;
    fn get_mut_h(&mut self) -> &mut Array1<f32>;
    fn get_mut_m_inv(&mut self) -> &mut Array1<f32>;
    fn get_mut_rad(&mut self) -> &mut Array1<f32>;
    fn get_mut_fx(&mut self) -> &mut Array1<f32>;
    fn get_mut_fy(&mut self) -> &mut Array1<f32>;
}

impl Base for DemDiscrete{
    fn get_m(&self) -> &Array1<f32>{
        & self.m
    }
    fn get_x(&self) -> &Array1<f32>{
        & self.x
    }
    fn get_y(&self) -> &Array1<f32>{
        & self.y
    }
    fn get_u(&self) -> &Array1<f32>{
        & self.u
    }
    fn get_v(&self) -> &Array1<f32>{
        & self.v
    }
    fn get_omega(&self) -> &Array1<f32>{
        & self.omega
    }
    fn get_inertia(&self) -> &Array1<f32>{
        & self.inertia
    }
    fn get_h(&self) -> &Array1<f32>{
        & self.h
    }
    fn get_m_inv(&self) -> &Array1<f32>{
        & self.m_inv
    }
    fn get_rad(&self) -> &Array1<f32>{
        & self.rad
    }
    fn get_fx(&self) -> &Array1<f32>{
        & self.fx
    }
    fn get_fy(&self) -> &Array1<f32>{
        & self.fy
    }
    fn get_id(&self) -> &usize{
        & self.id
    }
    fn get_mut_m(&mut self) -> &mut Array1<f32>{
        &mut self.m
    }
    fn get_mut_x(&mut self) -> &mut Array1<f32>{
        &mut self.x
    }
    fn get_mut_y(&mut self) -> &mut Array1<f32>{
        &mut self.y
    }
    fn get_mut_u(&mut self) -> &mut Array1<f32>{
        &mut self.u
    }
    fn get_mut_v(&mut self) -> &mut Array1<f32>{
        &mut self.v
    }
    fn get_mut_omega(&mut self) -> &mut Array1<f32>{
        &mut self.omega
    }
    fn get_mut_inertia(&mut self) -> &mut Array1<f32>{
        &mut self.inertia
    }
    fn get_mut_h(&mut self) -> &mut Array1<f32>{
        &mut self.h
    }
    fn get_mut_m_inv(&mut self) -> &mut Array1<f32>{
        &mut self.m_inv
    }
    fn get_mut_rad(&mut self) -> &mut Array1<f32>{
        &mut self.rad
    }
    fn get_mut_fx(&mut self) -> &mut Array1<f32>{
        &mut self.fx
    }
    fn get_mut_fy(&mut self) -> &mut Array1<f32>{
        &mut self.fy
    }
}
