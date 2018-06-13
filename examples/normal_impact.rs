#![allow(dead_code)]
#![allow(unused_imports)]

extern crate dem;
#[macro_use]
extern crate ndarray;

use dem::DemDiscrete;
use dem::contact_search::LinkedListGrid;
use dem::dem::{body_force_dem, make_forces_zero, spring_force};
use dem::geometry::dam_break_2d_geometry;
use dem::integrator::integrate;
use dem::save_data::{create_output_directory, dump_output};
use ndarray::prelude::*;

pub struct SimulationData {
    pub radius: f32,
}

impl SimulationData {
    fn new() -> Self {
        SimulationData { radius: 0.3 }
    }
}

fn setup_particle_properties(part1: &mut DemDiscrete, u: Array1<f32>, h: f32, mass: f32) {
    let m_inv = 1. / mass;
    for i in 0..part1.len {
        part1.h[i] = h;
        part1.u[i] = u[i];
        part1.rad[i] = h;
        part1.m[i] = mass;
        part1.m_inv[i] = m_inv;
    }
}

fn main() {
    let sim_data = SimulationData::new();

    let xa = array![-1., 1.];
    let ya = array![0., 0.];
    let u = array![2., -2.];

    let mut free = DemDiscrete::new_x_y(xa, ya, 0, "free".to_string());
    setup_particle_properties(
        &mut free,
        u,
        sim_data.radius,
        1000. * 4. * sim_data.radius.powf(2.),
    );

    let dt = 1e-4;
    let tf = 3000. * dt;
    let mut t = 0.;
    let mut time_step_number = 0;
    let scale = 2.;

    create_output_directory();
    while t < tf {
        let grid = LinkedListGrid::new(&mut vec![&mut free], scale);
        make_forces_zero(&mut free);
        spring_force(&mut vec![&mut free], 0, vec![0], 1e4, &grid);
        integrate(&mut free, dt);
        t = t + dt;
        println!("{:?}", t);
        if time_step_number % 100 == 0{
            dump_output(&mut vec![&mut free], time_step_number);
        }
        time_step_number += 1;
    }
}
