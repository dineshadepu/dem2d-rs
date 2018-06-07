extern crate ndarray;
extern crate rudem;

use rudem::{DemDiscrete, base::Base};
use ndarray::prelude::*;


#[test]
fn create_a_new_demdiscrete_from_len_and_id(){
    let len = 3;
    let id = 0;
    let ent1 = DemDiscrete::new(len, id);

    let expected = arr1(&[0., 0., 0.]);
    assert_eq!(ent1.x, expected);
}

#[test]
fn create_a_new_demdiscrete_from_x_and_id(){
    let x = Array::range(0., 1., 0.3);
    let id = 0;
    let ent1 = DemDiscrete::new_x(x.clone(), id);
    let expected = Array::range(0., 1., 0.3);

    // check get_x function and x attribute of entity
    assert_eq!(*ent1.get_x(), expected);

    // check get_fy function and see if it is zero
    assert_eq!(*ent1.get_fy(), Array::zeros(x.len()));

    // check id of the entity
    assert_eq!(*ent1.get_id(), 0);
}

#[test]
fn create_a_new_demdiscrete_from_x_y_and_id(){
    let x = Array::range(0., 1., 0.3);
    let y = Array::range(0., 1., 0.3);
    let id = 0;
    let ent1 = DemDiscrete::new_x_y(x.clone(), y.clone(), id);
    let expected = Array::range(0., 1., 0.3);

    // check get_x function and x attribute of entity
    assert_eq!(*ent1.get_x(), expected);
    assert_eq!(*ent1.get_y(), expected);

    // check get_fy function and see if it is zero
    assert_eq!(*ent1.get_fy(), Array::zeros(x.len()));

    // check id of the entity
    assert_eq!(*ent1.get_id(), 0);
}
