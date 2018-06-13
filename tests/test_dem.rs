extern crate dem;
#[macro_use]
extern crate ndarray;

use dem::{DemDiscrete, base::Base};
use dem::contact_search::LinkedListGrid;
use dem::dem as d;
use ndarray::prelude::*;

#[test]
fn check_spring_force_oblique_impact(){
    // ------------------------------------------------
    // ------------------------------------------------
    // check force when two particles are colliding in oblique fashion
    let mut ent1 = DemDiscrete::new_x_y(array![0.], array![0.0], 1, String::from("ent1"));
    ent1.h = array![1.];
    ent1.rad = array![1.];
    let mut ent2 = DemDiscrete::new_x_y(array![-1.5], array![1.0], 2, String::from("ent2"));
    ent2.h = array![1.];
    ent2.rad = array![1.];

    let scale = 2.;
    let grid = LinkedListGrid::new(&mut vec![&mut ent1, &mut ent2], scale);

    d::spring_force(&mut vec![&mut ent1, &mut ent2], 0, vec![1], 1., &grid);

    // the force in in normal direction with out damping is
    // f_n = k * overlap * \vec{n}
    assert!(ent1.fx[0] > 0.0);
    assert!(ent1.fy[0] < 0.0);
}

#[test]
fn check_spring_force_impact_with_different_entity(){
    // ------------------------------------------------
    // ------------------------------------------------
    // check force when two particles are colliding and are in different entity
    let mut ent1 = DemDiscrete::new_x_y(array![0., 2.0], array![0.0, 0.0], 1, String::from("ent1"));
    ent1.h = array![1., 1.];
    ent1.rad = array![1., 1.];
    let mut ent2 = DemDiscrete::new_x_y(array![0., 2.0], array![1.8, 1.8], 2, String::from("ent2"));
    ent2.h = array![1., 1.];
    ent2.rad = array![1., 1.];

    let scale = 2.;
    let grid = LinkedListGrid::new(&mut vec![&mut ent1, &mut ent2], scale);

    d::spring_force(&mut vec![&mut ent1, &mut ent2], 0, vec![1], 1., &grid);

    // the force in in normal direction with out damping is
    // f_n = k * overlap * \vec{n}
    assert!(ent1.fx[0] == 0.0);
    assert!(ent1.fy[0] < 0.0);
    assert!(ent1.fx[1] == 0.0);
    assert!(ent1.fy[1] < 0.0);
}

#[test]
fn check_spring_force_horizontal_impact(){
    // ------------------------------------------------
    // ------------------------------------------------
    // check force when two particles are overlapping only in horizontal direction
    let mut ent1 = DemDiscrete::new_x_y(array![0.], array![0.0], 1, String::from("ent1"));
    ent1.h = array![1.];
    ent1.rad = array![1.];
    let mut ent2 = DemDiscrete::new_x_y(array![-1.8], array![0.0], 2, String::from("ent2"));
    ent2.h = array![1.];
    ent2.rad = array![1.];

    let scale = 2.;
    let grid = LinkedListGrid::new(&mut vec![&mut ent1, &mut ent2], scale);

    d::spring_force(&mut vec![&mut ent1, &mut ent2], 0, vec![1], 1., &grid);

    // the force in in normal direction with out damping is
    // f_n = k * overlap * \vec{n}
    // here the overlap amount is 0.2
    // so f_n = 1. * 0.2 * [1., 0]
    assert!(ent1.fx[0] - 0.2 < 1e-6);
    assert!(ent1.fy[0] == 0.0);
}



#[test]
fn src_dest_index_test_in_equation(){
    let mut ent1 = DemDiscrete::new_x_y(array![0.], array![0.0], 1, String::from("ent1"));
    ent1.h = array![1.];
    ent1.rad = array![1.];
    let mut ent2 = DemDiscrete::new_x_y(array![-1.8], array![0.0], 2, String::from("ent2"));
    ent2.h = array![1.];
    ent2.rad = array![1.];
    let mut ent3 = DemDiscrete::new_x_y(array![1.8], array![0.0], 3, String::from("ent3"));
    ent3.h = array![1.];
    ent3.rad = array![1.];

    let scale = 2.;
    let grid = LinkedListGrid::new(&mut vec![&mut ent1, &mut ent2, &mut ent3], scale);

    // force due to entity 2 on entity 1 has to be positive in x direction
    d::spring_force(&mut vec![&mut ent3, &mut ent2, &mut ent1], 2, vec![1], 1., &grid);
    assert!(ent1.fx[0] > 0.);
    // make sure that the src entity doesn't get any forces
    assert!(ent2.fx[0] == 0.);

    // now to check force due to particle on the rights hand side of
    // entity 1, i.e force due to entity 3, make the forces acquired
    // due to entity 2 on entity 1 to zero
    d::make_forces_zero(&mut ent1);
    assert!(ent1.fx[0] == 0.);
    // now evaluate spring force
    d::spring_force(&mut vec![&mut ent3, &mut ent2, &mut ent1], 2, vec![0], 1., &grid);
    assert!(ent1.fx[0] < 0.);
    // make sure that the src entity doesn't get any forces
    assert!(ent3.fx[0] == 0.);
}

#[test]
fn test_full_simulation(){
    // check with dem entities
    let mut ent1 = DemDiscrete::new_x_y(array![0., 2.2], array![2.01, 2.01], 1, String::from("ent1"));
    ent1.rad = array![1., 1.];
    ent1.h = array![1., 1.];
    ent1.v = array![-10., -10.];
    let mut ent2 = DemDiscrete::new_x_y(array![0., 2.2], array![0.0, 0.0], 1, String::from("ent2"));
    ent1.rad = array![1., 1.];
    ent1.h = array![1., 1.];


    let dt = 1e-4;
    let tf = 20. * dt;
    let mut t = 0.;
    let scale = 2.;
}


#[test]
fn test_get_mut_two(){
    // check simple float vector slicing
    let mut  v = vec![1., 2., 3., 4.];
    let (a, b) = d::get_two_mut(&mut v, 2, 1);
    assert_eq!(3., *a);
    assert_eq!(2., *b);

    // check with dem entities
    let mut ent1 = DemDiscrete::new_x(Array::range(0., 1., 0.3), 0, String::from("ent1"));
    let mut ent2 = DemDiscrete::new_x(Array::range(0., 1., 0.3), 1, String::from("ent2"));
    let mut ent3 = DemDiscrete::new_x(Array::range(0., 1., 0.3), 2, String::from("ent3"));
    let mut ent4 = DemDiscrete::new_x(Array::range(0., 1., 0.3), 3, String::from("ent4"));

    let mut v = vec![&mut ent1, &mut ent2, &mut ent3, &mut ent4];
    let (a, b) = d::get_two_mut(&mut v, 2, 1);
    assert_eq!(2, a.id);
    assert_eq!(1, b.id);
    assert_eq!(String::from("ent3"), a.name);
    assert_eq!(String::from("ent2"), b.name);
}
