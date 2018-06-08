use super::DemDiscrete;
use base::{Base, BasePartsMut};
use contact_search::{LinkedListGrid, get_neighbours_ll};

pub fn body_force_dem<T: Base>(entity: &mut T, gx: f32, gy: f32) {
    let ent1 = entity.get_parts_mut();

    for i in 0..*ent1.len {
        ent1.fx[i] += ent1.m[i] * gx;
        ent1.fy[i] += ent1.m[i] * gy;
    }
}

pub fn spring_force<T: Base>(
    entities: &mut Vec<&mut T>,
    dst_id: usize,
    srcs: Vec<usize>,
    kn: f32,
    grid: LinkedListGrid,
)
{
    let dst = entities[dst_id].get_parts_mut();
    for src_id in srcs{
        if dst_id == src_id{
            for i in 0..*dst.len{
                let nbrs = get_neighbours_ll([dst.x[i], dst.y[i]], &grid, &src_id);
                println!("{:?}", nbrs);
            }
        }
    }
}
