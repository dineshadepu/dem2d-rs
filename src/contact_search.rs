use std::collections::HashMap;
use std::collections::LinkedList;

use DemDiscrete;
use arr1;
use base::{Base, BasePartsMut};

#[derive(Debug, Clone)]
pub struct CellGrid {
    pub indices: HashMap<usize, LinkedList<usize>>,
}

impl CellGrid {
    pub fn new(keys: &Vec<usize>) -> Self {
        let mut cell = CellGrid {
            indices: HashMap::new(),
        };
        for key in keys {
            cell.indices.insert(*key, LinkedList::new());
        }
        cell
    }
}

#[derive(Debug)]
pub struct LinkedListGrid {
    pub no_x_cells: usize,
    pub no_y_cells: usize,
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub size: f32,
    pub cells: Vec<CellGrid>,
}

impl LinkedListGrid {
    pub fn new<T: Base>(world: &mut Vec<&mut T>, scale: f32) -> LinkedListGrid {
        // compute the limits of the grid
        let mut x_min = world[0].get_x()[0];
        let mut x_max = world[0].get_x()[0];
        let mut y_min = world[0].get_y()[0];
        let mut y_max = world[0].get_y()[0];
        // find particle with maximum size to set
        // the size of the grid cell
        let mut size = 0.;

        for i in 0..world.len() {
            let ent_i = &world[i].get_parts_mut();
            for i in 0..ent_i.x.len() {
                if x_min > ent_i.x[i] {
                    x_min = ent_i.x[i];
                }
                if x_max < ent_i.x[i] {
                    x_max = ent_i.x[i];
                }
                if y_min > ent_i.y[i] {
                    y_min = ent_i.y[i];
                }
                if y_max < ent_i.y[i] {
                    y_max = ent_i.y[i];
                }
                if size < ent_i.h[i] {
                    size = ent_i.h[i];
                }
            }
        }
        // scale the size
        size = size * scale;
        // increase the size of the grid by changing
        // the limits
        x_min = x_min - size / 10.;
        x_max = x_max + size / 10.;
        y_min = y_min - size / 10.;
        y_max = y_max + size / 10.;

        // number of cells in x direction and y direction
        let no_x_cells = ((x_max - x_min) / size) as usize + 1;
        let no_y_cells = ((y_max - y_min) / size) as usize + 1;

        // get all keys of the entities
        let mut keys: Vec<usize> = vec![];
        for i in 0..world.len() {
            let ent_i = world[i].get_parts_mut();
            keys.push(*ent_i.id);
        }

        // create cells of required size
        let mut cells: Vec<CellGrid> = vec![CellGrid::new(&keys); no_x_cells * no_y_cells];

        for j in 0..world.len() {
            let entity = world[j].get_parts_mut();
            let id = entity.id;
            for i in 0..entity.x.len() {
                // find the index
                let x_index = ((entity.x[i] - x_min) / size) as usize;
                let y_index = ((entity.y[i] - y_min) / size) as usize;
                // one dimentional index is
                let index = x_index * no_y_cells + y_index;
                cells[index].indices.get_mut(&id).unwrap().push_back(i);
            }
        }
        let grid = LinkedListGrid {
            no_x_cells,
            no_y_cells,
            x_min,
            x_max,
            y_min,
            y_max,
            size,
            cells,
        };

        grid
    }
}

pub fn get_neighbours_ll(
    pos: [f32; 2],
    grid: &LinkedListGrid,
    src_id: &usize,
) -> LinkedList<usize> {
    let cells = &grid.cells;
    let cells_len = cells.len();

    let x_index = ((pos[0] - grid.x_min) / grid.size) as usize;
    let y_index = ((pos[1] - grid.y_min) / grid.size) as usize;

    // index in grid
    let index = x_index * grid.no_y_cells + y_index;

    let mut neighbours_particle: LinkedList<usize> = LinkedList::new();

    if index >= 0 {
        for val in &cells[index].indices[src_id] {
            neighbours_particle.push_front(*val);
        }
    }
    if let Some(j) = index.checked_sub(1) {
        // make sure that the index is in limits of cell
        if j <= cells_len - 1 {
            for val in &cells[j].indices[src_id] {
                neighbours_particle.push_front(*val);
            }
        }
    }
    if let Some(j) = index.checked_add(1) {
        // make sure that the index is in limits of cell
        if j <= cells_len - 1 {
            for val in &cells[j].indices[src_id] {
                neighbours_particle.push_front(*val);
            }
        }
    }
    if let Some(j) = index.checked_sub(grid.no_y_cells) {
        // make sure that the index is in limits of cell
        if j <= cells_len - 1 {
            for val in &cells[j].indices[src_id] {
                neighbours_particle.push_front(*val);
            }
        }
    }
    if let Some(j) = index.checked_sub(grid.no_y_cells - 1) {
        // make sure that the index is in limits of cell
        if j <= cells_len - 1 {
            for val in &cells[j].indices[src_id] {
                neighbours_particle.push_front(*val);
            }
        }
    }
    if let Some(j) = index.checked_sub(grid.no_y_cells + 1) {
        // make sure that the index is in limits of cell
        if j <= cells_len - 1 {
            for val in &cells[j].indices[src_id] {
                neighbours_particle.push_front(*val);
            }
        }
    }

    if let Some(j) = index.checked_add(grid.no_y_cells) {
        // make sure that the index is in limits of cell
        if j <= cells_len - 1 {
            for val in &cells[j].indices[src_id] {
                neighbours_particle.push_front(*val);
            }
        }
    }
    if let Some(j) = index.checked_add(grid.no_y_cells - 1) {
        // make sure that the index is in limits of cell
        if j <= cells_len - 1 {
            for val in &cells[j].indices[src_id] {
                neighbours_particle.push_front(*val);
            }
        }
    }
    if let Some(j) = index.checked_add(grid.no_y_cells + 1) {
        // make sure that the index is in limits of cell
        if j <= cells_len - 1 {
            for val in &cells[j].indices[src_id] {
                neighbours_particle.push_front(*val);
            }
        }
    }
    neighbours_particle
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn create_nine_particles() -> DemDiscrete {
        let x = vec![0.1, 0.2, 0.3, 0.1, 0.2, 0.3, 0.1, 0.2, 0.3];
        let y = vec![0.1, 0.1, 0.1, 0.2, 0.2, 0.2, 0.3, 0.3, 0.3];
        let h = vec![0.05; x.len()];

        let entity = DemDiscrete::new_x_y_h(arr1(&x), arr1(&y), arr1(&h), 0);
        entity
    }

    #[test]
    fn create_ine_particles(){
        let mut entity = create_nine_particles();
        let grid = LinkedListGrid::new(&mut vec![&mut entity], 1.0);
        println!("{:?}", grid);
    }
    // #[test]
    fn test_grid_attributes_nine_particles() {
        let mut entity = create_nine_particles();
        let grid = LinkedListGrid::new(&mut vec![&mut entity], 1.0);

        assert!(grid.x_min - 0.09 < 1e-8);
        assert!(grid.x_max - 0.31 < 1e-8);
        assert_eq!(grid.no_x_cells, 3);
        assert_eq!(grid.no_y_cells, 3);

        // check if first particle is in correct cell
        assert_eq!(grid.cells[0].indices[&0].front(), Some(&0));
        assert_eq!(grid.cells[1].indices[&0].front(), Some(&3));
        assert_eq!(grid.cells[2].indices[&0].front(), Some(&6));
        assert_eq!(grid.cells[3].indices[&0].front(), Some(&1));
        assert_eq!(grid.cells[4].indices[&0].front(), Some(&4));
        assert_eq!(grid.cells[5].indices[&0].front(), Some(&7));
        assert_eq!(grid.cells[6].indices[&0].front(), Some(&2));
        assert_eq!(grid.cells[7].indices[&0].front(), Some(&5));
        assert_eq!(grid.cells[8].indices[&0].front(), Some(&8));
    }
    // #[test]
    fn test_neighbours_nine_particles() {
        let mut entity = create_nine_particles();
        let grid = LinkedListGrid::new(&mut vec![&mut entity], 1.0);

        // the zero'th particles neighbours are [4, 3, 1, 0]
        let zero_nbrs = get_neighbours_ll([entity.x[0], entity.y[0]], &grid, &0);
        let expected = vec![4, 3, 1, 0, 6];
        // check total neighbours length
        assert_eq!(expected.len(), zero_nbrs.len());
        // check each neighbour index
        for i in expected {
            assert_eq!(zero_nbrs.contains(&i), true);
        }

        // the 4'th particles neighbours are [8, 2, 5, 0, 6, 3, 7, 1, 4]
        let fourth_nbrs = get_neighbours_ll([entity.x[4], entity.y[4]], &grid, &0);
        let expected = vec![8, 2, 5, 0, 6, 3, 7, 1, 4];
        // check total neighbours length
        assert_eq!(expected.len(), fourth_nbrs.len());
        // check each neighbour index
        for i in expected {
            assert_eq!(fourth_nbrs.contains(&i), true);
        }
        // the 2'th particles neighbours are [8, 2, 5, 0, 6, 3, 7, 1, 4]
        let second_nbrs = get_neighbours_ll([entity.x[2], entity.y[2]], &grid, &0);
        let expected = vec![2, 5, 7, 1, 6, 4, 8];
        // check total neighbours length
        assert_eq!(expected.len(), second_nbrs.len());
        // check each neighbour index
        for i in expected {
            assert_eq!(second_nbrs.contains(&i), true);
        }
    }
}
