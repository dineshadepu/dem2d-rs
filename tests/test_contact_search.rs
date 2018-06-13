extern crate dem;
#[macro_use]
extern crate ndarray;

use dem::contact_search::{LinkedListGrid, get_neighbours_ll};
use dem::{DemDiscrete, base::Base};
use dem::dem as d;
use ndarray::prelude::*;

#[test]
fn test_grid_cells_indices(){
    //
    //
    //
    //
    //           |-------------------------------------------|
    //           |          |         |          |           |
    //           |          |         |          |           |
    //           |          |         |          |           |
    //           |          |         |          |           |
    //           |__________|_________|__________|___________|
    //           |          |         |          |           |
    //           |          | rad 1   |          |           |
    //           |          |_________|          |           |
    //           |          |         |          |           |
    //           |__________|_________|__________|___________|
    //           |          |         |          |           |
    //           |          |         |          |           |
    //           | 2        |         | 2        |           |
    //           |. id 2    |         |. id 2    |           |
    //           |__________|_________|__________|___________|
    //           |          |         |          |           |
    //           |          |         |          |           |
    //           |          |         |          |           |
    //           | 1        |         | 1        |           |
    //           |. id 1    |         |.  id 2   |           |
    //           |-------------------------------------------|
    // check with dem entities
    let mut ent1 = DemDiscrete::new_x_y(array![0., 2.], array![0., 0.], 1, String::from("ent1"));
    ent1.h = array![1., 1.];
    let mut ent2 = DemDiscrete::new_x_y(array![0., 2.], array![2.0, 2.0], 2, String::from("ent2"));
    ent2.h = array![1., 1.];

    // neighbours for a grid size of h=1., same as radius of particle
    let scale = 1.;
    let grid = LinkedListGrid::new(&mut vec![&mut ent1, &mut ent2], scale);

    let nbrs = get_neighbours_ll([ent1.x[0], ent1.y[0]], &grid,  &2);
    assert_eq!(0, nbrs.len());
    let nbrs = get_neighbours_ll([ent1.x[0], ent1.y[0]], &grid,  &1);
    assert_eq!(1, nbrs.len());

    // neighbours for a grid size of 2 * h=2., double the radius
    let scale = 2.;
    let grid = LinkedListGrid::new(&mut vec![&mut ent1, &mut ent2], scale);

    let nbrs = get_neighbours_ll([ent1.x[0], ent1.y[0]], &grid,  &2);
    assert_eq!(2, nbrs.len());
    let nbrs = get_neighbours_ll([ent1.x[0], ent1.y[0]], &grid,  &1);
    assert_eq!(2, nbrs.len());
}

#[test]
fn test_grid_cells_indices_a_corner_case(){
    // check a corner case:
    // every particle is in single cell. Make sure that indices are not repeating
    // check with dem entities
    let mut ent1 = DemDiscrete::new_x_y(array![0., 0.1, 0.], array![0., 0., 0.1], 1, String::from("ent1"));
    ent1.h = array![1., 1., 1.];
    let mut ent2 = DemDiscrete::new_x_y(array![-0.1, 0., 0.1], array![0., -0.1, 0.1], 2, String::from("ent1"));
    ent2.h = array![1., 1., 1.];

    // neighbours for a grid size of h=1., same as radius of particle
    let scale = 1.;
    let grid = LinkedListGrid::new(&mut vec![&mut ent1, &mut ent2], scale);

    let nbrs = get_neighbours_ll([ent1.x[0], ent1.y[0]], &grid,  &2);
    assert_eq!(3, nbrs.len());
    let nbrs = get_neighbours_ll([ent1.x[0], ent1.y[0]], &grid,  &1);
    assert_eq!(3, nbrs.len());

    let expected_nbrs = vec![0, 1, 2];
    for i in expected_nbrs{
        assert_eq!(true, nbrs.contains(&i));
    }
}