extern crate dem;
#[macro_use]
extern crate ndarray;

use dem::{DemDiscrete, base::Base};
use dem::contact_search::LinkedListGrid;
use dem::dem as d;
use ndarray::prelude::*;

#[test]
fn create_a_new_demdiscrete_from_len_and_id() {
    let len = 3;
    let id = 0;
    let ent1 = DemDiscrete::new(len, id, "ent1".to_string());

    let expected = arr1(&[0., 0., 0.]);
    assert_eq!(ent1.x, expected);
}

#[test]
fn create_a_new_demdiscrete_from_x_and_id() {
    let x = Array::range(0., 1., 0.3);
    let id = 0;
    let mut ent1 = DemDiscrete::new_x(x.clone(), id, String::from("ent1"));
    let expected = Array::range(0., 1., 0.3);

    // check get_x function and x attribute of entity
    let mut_ent1 = ent1.get_parts_mut();
    assert_eq!(*mut_ent1.x, expected);

    // check get_fy function and see if it is zero
    assert_eq!(*mut_ent1.fy, Array::zeros(x.len()));

    // check id of the entity
    assert_eq!(*mut_ent1.id, 0);
}

#[test]
fn create_a_new_demdiscrete_from_x_y_and_id() {
    let x = Array::range(0., 1., 0.3);
    let y = Array::range(0., 1., 0.3);
    let id = 0;
    let mut ent1 = DemDiscrete::new_x_y(x.clone(), y.clone(), id, String::from("ent1"));
    let expected = Array::range(0., 1., 0.3);

    // check get_mut_parts function of trait and x attribute of entity
    let mut_ent1 = ent1.get_parts_mut();
    assert_eq!(*mut_ent1.x, expected);
    assert_eq!(*mut_ent1.y, expected);

    // check get_fy function and see if it is zero
    assert_eq!(*mut_ent1.fy, Array::zeros(x.len()));

    // check id of the entity
    assert_eq!(*mut_ent1.id, 0);
}


#[test]
fn check_spring_force(){
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
