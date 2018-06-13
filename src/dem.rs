use super::DemDiscrete;
use base::Base;
use contact_search::{get_neighbours_ll, LinkedListGrid};

pub fn make_forces_zero<T: Base>(entity: &mut T) {
    let ent1 = entity.get_parts_mut();

    for i in 0..*ent1.len {
        ent1.fx[i] = 0.;
        ent1.fy[i] = 0.;
    }
}

pub fn body_force_dem<T: Base>(entity: &mut T, gx: f32, gy: f32) {
    let ent1 = entity.get_parts_mut();

    for i in 0..*ent1.len {
        ent1.fx[i] += ent1.m[i] * gx;
        ent1.fy[i] += ent1.m[i] * gy;
    }
}

pub fn get_two_mut<T>(data: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {

    assert!(a != b);

    let ptr: *mut [T] = data;

    unsafe {

        (&mut (*ptr)[a], &mut (*ptr)[b])

    }

}

pub fn spring_force<T: Base>(
    mut entities: &mut Vec<&mut T>,
    dst_id: usize,
    srcs: Vec<usize>,
    kn: f32,
    grid: &LinkedListGrid,
) {
    for src_id in srcs {
        if dst_id == src_id {
            let dst = entities[dst_id].get_parts_mut();
            for i in 0..*dst.len {
                let nbrs = get_neighbours_ll([dst.x[i], dst.y[i]], &grid, &dst.id);
                for j in nbrs {
                    if i != j {
                        let dx = dst.x[i] - dst.x[j];
                        let dy = dst.y[i] - dst.y[j];
                        let dist = (dx.powf(2.) + dy.powf(2.)).powf(0.5);
                        let overlap = dst.rad[i] + dst.rad[j] - dist;

                        if overlap > 0. {
                            let nx = dx / dist;
                            let ny = dy / dist;
                            dst.fx[i] += kn * overlap * nx;
                            dst.fy[i] += kn * overlap * ny;
                        }
                    }
                }
            }
        } else {
            let (dst_main, src_main) = get_two_mut(&mut entities, dst_id, src_id);
            let dst = dst_main.get_parts_mut();
            let src = src_main.get_parts_mut();
            for i in 0..*dst.len {
                let nbrs = get_neighbours_ll([dst.x[i], dst.y[i]], &grid, &src.id);
                for j in nbrs {
                    let dx = dst.x[i] - src.x[j];
                    let dy = dst.y[i] - src.y[j];
                    let dist = (dx.powf(2.) + dy.powf(2.)).powf(0.5);
                    let overlap = dst.rad[i] + src.rad[j] - dist;
                    if overlap > 0. {
                        let nx = dx / dist;
                        let ny = dy / dist;
                        dst.fx[i] += kn * overlap * nx;
                        dst.fy[i] += kn * overlap * ny;
                    }
                }
            }
        }
    }
}
