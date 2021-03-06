use std::collections::HashMap;

pub struct NNPSMutParts<'a> {
    pub len: &'a mut usize,
    pub x: &'a mut Vec<f32>,
    pub y: &'a mut Vec<f32>,
    pub h: &'a mut Vec<f32>,
    pub id: &'a mut usize,
}

// trait which has to be implemented by every struct which need to be
// implemented linked list neighbour search
pub trait NNPS {
    fn get_parts_mut_nnps(&mut self) -> NNPSMutParts;
    fn get_x(&self) -> &Vec<f32>;
    fn get_y(&self) -> &Vec<f32>;
}

#[macro_export]
macro_rules! impl_nnps{
    ($($t:ty)*) => ($(
        impl NNPS for $t {
            fn get_parts_mut_nnps(&mut self) -> NNPSMutParts {
                NNPSMutParts{
                    len: &mut self.len,
                    x: &mut self.x,
                    y: &mut self.y,
                    h: &mut self.h,
                    id: &mut self.id,
                }
            }
            fn get_x(&self) ->  & Vec<f32>{
                &self.x
            }
            fn get_y(&self) ->  & Vec<f32>{
                &self.y
            }
        }
    )*)
}

#[derive(Debug, Clone)]
pub struct CellGrid {
    pub indices: HashMap<usize, Vec<usize>>,
}

impl CellGrid {
    pub fn new(keys: &Vec<usize>) -> Self {
        let mut cell = CellGrid {
            indices: HashMap::new(),
        };
        for key in keys {
            cell.indices.insert(*key, Vec::new());
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
    pub fn new<T: NNPS>(world: &mut Vec<&mut T>, scale: f32) -> LinkedListGrid {
        // compute the limits of the grid
        let mut x_min = world[0].get_x()[0];
        let mut x_max = world[0].get_x()[0];
        let mut y_min = world[0].get_y()[0];
        let mut y_max = world[0].get_y()[0];
        // find particle with maximum size to set
        // the size of the grid cell
        let mut size = 0.;

        for i in 0..world.len() {
            let ent_i = &world[i].get_parts_mut_nnps();
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
        let no_x_cells = ((x_max - x_min) / size) as usize + 2;
        let no_y_cells = ((y_max - y_min) / size) as usize + 2;

        // get all keys of the entities
        let mut keys: Vec<usize> = vec![];
        for i in 0..world.len() {
            let ent_i = world[i].get_parts_mut_nnps();
            keys.push(*ent_i.id);
        }

        // create cells of required size
        let mut cells: Vec<CellGrid> = vec![CellGrid::new(&keys); no_x_cells * no_y_cells];

        for j in 0..world.len() {
            let entity = world[j].get_parts_mut_nnps();
            let id = entity.id;
            for i in 0..entity.x.len() {
                // find the index
                let x_index = ((entity.x[i] - x_min) / size) as usize;
                let y_index = ((entity.y[i] - y_min) / size) as usize;
                // one dimentional index is
                let index = x_index * no_y_cells + y_index;
                cells[index].indices.get_mut(&id).unwrap().push(i);
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

pub fn get_neighbours_ll<'a>(
    pos: [f32; 3],
    grid: &'a LinkedListGrid,
    src_id: &usize,
) -> Vec<&'a Vec<usize>> {
    let cells = &grid.cells;

    let x_index = ((pos[0] - grid.x_min) / grid.size) as usize;
    let y_index = ((pos[1] - grid.y_min) / grid.size) as usize;

    // index in grid
    let index = x_index * grid.no_y_cells + y_index;

    let mut neighbours_particle: Vec<&Vec<usize>> = Vec::new();

    // for the stack of z = 0
    for neighbour in &[
        Some(index),
        index.checked_sub(1),
        index.checked_add(1),
        index.checked_sub(grid.no_y_cells),
        index.checked_sub(grid.no_y_cells - 1),
        index.checked_sub(grid.no_y_cells + 1),
        index.checked_add(grid.no_y_cells),
        index.checked_add(grid.no_y_cells - 1),
        index.checked_add(grid.no_y_cells + 1),
    ] {
        if let Some(cell) = neighbour.and_then(|index| cells.get(index)) {
            neighbours_particle.push(&cell.indices[src_id])
        }
    }
    neighbours_particle
}
