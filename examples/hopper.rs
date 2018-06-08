extern crate rudem;
extern crate ndarray;

use ndarray::prelude::*;
use rudem::{DemDiscrete};
use rudem::geometry::dam_break_2d_geometry;
use rudem::dem::body_force;

use std::fs;


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

fn main(){
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

    let grains = DemDiscrete::new_x_y(arr1(&xg), arr1(&yg), 0);
    let tank = DemDiscrete::new_x_y(arr1(&xt), arr1(&yt), 1);

    let dt = 1e-3;
    let t = 100.*dt;

    let result = fs::create_dir("./data");

    let mut file_number = 0;
    // fs::create_dir_all("/data")?;
    while t < tf {
        body_force(&mut grains);
        t = t + dt;
    }

}
