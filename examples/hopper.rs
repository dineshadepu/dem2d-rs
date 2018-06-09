#![allow(dead_code)]
#![allow(unused_imports)]

extern crate ndarray;
extern crate dem;

use ndarray::prelude::*;
use dem::DemDiscrete;
use dem::contact_search::LinkedListGrid;
use dem::dem::{body_force_dem, spring_force};
use dem::geometry::dam_break_2d_geometry;

pub struct SimulationData {
    pub grains_spacing: f32,
    pub grains_length: f32,
    pub grains_height: f32,
    pub tank_spacing: f32,
    pub tank_length: f32,
    pub tank_height: f32,
    pub tank_layers: usize,
}

impl SimulationData {
    fn new() -> Self {
        SimulationData {
            grains_spacing: 0.1,
            grains_length: 4.,
            grains_height: 5.,
            tank_spacing: 0.1,
            tank_length: 10.,
            tank_height: 7.,
            tank_layers: 2,
        }
    }
}

fn setup_particle_properties(part1: &mut DemDiscrete, h: f32){
    for i in 0..part1.len {
        part1.h[i] = h;
    }
}

fn main() {
    let sim_data = SimulationData::new();

    let (xg, yg, xt, yt) = dam_break_2d_geometry(
        sim_data.grains_length,
        sim_data.grains_height,
        sim_data.grains_spacing,
        sim_data.tank_length,
        sim_data.tank_height,
        sim_data.tank_spacing,
        sim_data.tank_layers,
    );

    let mut grains = DemDiscrete::new_x_y(arr1(&xg), arr1(&yg), 0);
    let mut tank = DemDiscrete::new_x_y(arr1(&xt), arr1(&yt), 1);
    setup_particle_properties(&mut grains, sim_data.grains_spacing);
    setup_particle_properties(&mut tank, sim_data.tank_spacing);

    let dt = 1e-3;
    let tf = 1. * dt;
    let mut t = 0.;
    let scale = 1.;

    while t < tf {
        let grid = LinkedListGrid::new(&mut vec![&mut grains, &mut tank], scale);
        body_force_dem(&mut grains, 0., -9.81);
        spring_force(&mut vec![&mut grains, &mut tank], 0, vec![0, 1], 1e4, grid);
        t = t + dt;
        println!("{:?}", t);
    }
}
