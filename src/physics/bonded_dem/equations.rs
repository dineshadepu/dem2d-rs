// local imports
use super::{Bond, DemBondedDstTrait};
use contact_search::{get_neighbours_ll, LinkedListGrid};
use math::distance;

// external crate imports
use cm::{Vector3 as V3, Zero};

/// Setup the DemBonded structure. Given the particle array, create the bonds of
/// each particle with it neighbours.  Here scale is multiplied by each
/// particles radius, and particles which are under such scaled radius are
/// considered to be bonded.
pub fn setup_bonded_structure<T: DemBondedDstTrait>(dest: &mut T, scale: f32) {
    // create the grid for neighbour search
    // this scale is different from bonded
    // scale. This is for neighbour particles
    let grid = LinkedListGrid::new(&mut vec![dest], 2.);

    // Get the particle array with mutable fields
    let dst = dest.get_parts_mut();
    for i in 0..*dst.len {
        // position of particle i
        let pos_i = V3::new(dst.x[i], dst.y[i], 0.);

        let nbrs = get_neighbours_ll([dst.x[i], dst.y[i], 0.], &grid, &dst.id);

        for sub_view in nbrs {
            // neighbour indices j
            for &j in sub_view {
                if i != j {
                    // position of particle j in source
                    let pos_j = V3::new(dst.x[j], dst.y[j], 0.);

                    let symmetric_scaled_radius = scale * (dst.rad[i] + dst.rad[j]);

                    // distance between particles
                    let dstnc = distance(&pos_i, &pos_j);

                    // if the distance between the particles is less than
                    // the symmetric radius, then the particles will be bonded
                    if dstnc < symmetric_scaled_radius {
                        dst.bonds[i].insert(j, Bond::new());
                        dst.bonds0[i].insert(j, Bond::new());
                    }
                }
            }
        }
    }
}

fn internal_force_bonded_dem<T: DemBondedDstTrait>(
    dest: &mut T,
    kn: f32,
    mu: f32,
) {
    let dst = dest.get_parts_mut();

    // Select the neighbours function according to the dimention
    for i in 0..*dst.len {
        // position of particle i
        let pos_i = V3::new(dst.x[i], dst.y[i], 0.);
        // linear velocity of particle i
        let vel_i = V3::new(dst.u[i], dst.v[i], 0.);
        // angular velocity of particle i
        let ang_vel_i = V3::new(0., 0., dst.omega_z[i]);

        // iterate over the bonds of particle i
        for (j, bond) in &mut dst.bonds[i] {
            println!("{}, {:?}", j, bond);
        }
    }
}
