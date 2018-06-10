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
use dem::save_data::create_output_directory;
use ndarray::prelude::*;

pub struct SimulationData {
    pub radius: f32,
}

impl SimulationData {
    fn new() -> Self {
        SimulationData { radius: 0.1 }
    }
}

fn setup_particle_properties(part1: &mut DemDiscrete, h: f32, mass: f32) {
    let m_inv = 1. / mass;
    for i in 0..part1.len {
        part1.h[i] = h;
        part1.rad[i] = h;
        part1.m[i] = mass;
        part1.m_inv[i] = m_inv;
    }
}

fn main() {
    let sim_data = SimulationData::new();

    let xa = array![0.];
    let ya = array![2.];
    let xb = array![0.];
    let yb = array![0.];

    let mut free = DemDiscrete::new_x_y(xa, ya, 0);
    let mut boundary = DemDiscrete::new_x_y(xb, yb, 1);
    setup_particle_properties(
        &mut free,
        sim_data.radius,
        1000. * 4. * sim_data.radius.powf(2.),
    );
    setup_particle_properties(
        &mut boundary,
        sim_data.radius,
        1000. * 4. * sim_data.radius.powf(2.),
    );

    let dt = 1e-4;
    let tf = 1000. * dt;
    let mut t = 0.;
    let scale = 2.;

    create_output_directory();
    while t < tf {
        let grid = LinkedListGrid::new(&mut vec![&mut free, &mut boundary], scale);
        make_forces_zero(&mut free);
        body_force_dem(&mut free, 0., -9.81);
        spring_force(&mut vec![&mut free, &mut boundary], 0, vec![1], 1e4, grid);
        integrate(&mut free, dt);
        t = t + dt;
        println!("{:?} {:?}", t, free.y[0]);
    }

    for i in 0..grains.x.len(){
        println!("{:?} {:?}", grains.x[i], grains.y[i]);
    }
}