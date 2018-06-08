use std::collections::HashMap;
use std::collections::LinkedList;

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
                if size < 2. * ent_i.h[i] {
                    size = 2. * ent_i.h[i];
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
        let mut cells: Vec<CellGrid> = vec![CellGrid::new(&keys); no_x_cells * no_x_cells];

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

pub fn get_neighbours_ll(pos: [f32; 2], grid: &LinkedListGrid, src_id: &usize) {
    let cells = &grid.cells;
    let cells_len = cells.len();

    let x_index = ((pos[0] - grid.x_min) / grid.size) as usize;
    let y_index = ((pos[1] - grid.y_min) / grid.size) as usize;
    println!("{:?}", x_index);

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
}
