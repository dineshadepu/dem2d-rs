use super::DemDiscrete;
use contact_search::{get_neighbours_ll, LinkedListGrid};

pub fn make_forces_zero(entity: &mut DemDiscrete) {
    for i in 0..entity.len {
        entity.fx[i] = 0.;
        entity.fy[i] = 0.;
    }
}

pub fn body_force_dem(entity: &mut DemDiscrete, gx: f32, gy: f32) {
    for i in 0..entity.len {
        entity.fx[i] += entity.m[i] * gx;
        entity.fy[i] += entity.m[i] * gy;
    }
}

pub fn get_two_mut<DemDiscrete>(
    data: &mut [DemDiscrete],
    a: usize,
    b: usize,
) -> (&mut DemDiscrete, &mut DemDiscrete) {
    assert!(a != b);

    let ptr: *mut [DemDiscrete] = data;

    unsafe { (&mut (*ptr)[a], &mut (*ptr)[b]) }
}

pub fn spring_force(
    mut entities: &mut Vec<&mut DemDiscrete>,
    dst_id: usize,
    srcs: Vec<usize>,
    kn: f32,
    grid: &LinkedListGrid,
) {
    for src_id in srcs {
        if dst_id == src_id {
            let dst = &mut entities[dst_id];
            for i in 0..dst.len {
                let nbrs = get_neighbours_ll([dst.x[i], dst.y[i], 0.], &grid, &dst.id);
                for sub_view in nbrs {
                    for &j in sub_view {
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
            }
        } else {
            let (dst, src) = get_two_mut(&mut entities, dst_id, src_id);
            for i in 0..dst.len {
                let nbrs = get_neighbours_ll([dst.x[i], dst.y[i], 0.], &grid, &src.id);
                for sub_view in nbrs {
                    for &j in sub_view {
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
}
