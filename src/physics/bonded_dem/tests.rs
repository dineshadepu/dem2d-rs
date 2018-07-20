use super::DemBonded;
use super::equations::setup_bonded_structure;

fn setup_particle_properties(part1: &mut DemBonded, x: Vec<f32>, y: Vec<f32>, h: f32) {
    for i in 0..part1.len {
        part1.x[i] = x[i];
        part1.y[i] = y[i];
        part1.h[i] = 1.2 * h;
        part1.rad[i] = h;
    }
}

#[test]
fn test1_setup_structure_of_bonded_dem() {
    // create four particles, on a single line and create a bonded dem entity.
    // set it up and check if the particles are bonded correctly
    let spacing = 1.0;
    let radius = spacing / 2.;
    let x = vec![0., 1.0, 2.0, 3.0];
    let y = vec![0., 0.0, 0.0, 0.0];
    let mut beam = DemBonded::new(x.len(), 0, "beam".to_string());
    setup_particle_properties(&mut beam, x, y, radius);

    setup_bonded_structure(&mut beam, 1.2);

    /* particles are alligned the following way

    0 - 1 - 2 - 3

    So particle 0 is aligned with 1
    So particle 1 is aligned with 0, 2
    So particle 2 is aligned with 1, 3
    So particle 3 is aligned with 2

    Let's check that
     */
    // particle 0
    assert_eq!(false, beam.bonds[0].contains_key(&0));
    assert_eq!(true, beam.bonds[0].contains_key(&1));
    assert_eq!(false, beam.bonds[0].contains_key(&2));
    assert_eq!(false, beam.bonds[0].contains_key(&3));

    // particle 1
    assert_eq!(true, beam.bonds[1].contains_key(&0));
    assert_eq!(false, beam.bonds[1].contains_key(&1));
    assert_eq!(true, beam.bonds[1].contains_key(&2));
    assert_eq!(false, beam.bonds[1].contains_key(&3));

    // particle 2
    assert_eq!(false, beam.bonds[2].contains_key(&0));
    assert_eq!(true, beam.bonds[2].contains_key(&1));
    assert_eq!(false, beam.bonds[2].contains_key(&2));
    assert_eq!(true, beam.bonds[2].contains_key(&3));

    // particle 3
    assert_eq!(false, beam.bonds[3].contains_key(&0));
    assert_eq!(false, beam.bonds[3].contains_key(&1));
    assert_eq!(true, beam.bonds[3].contains_key(&2));
    assert_eq!(false, beam.bonds[3].contains_key(&3));
}

#[test]
fn test2_setup_structure_of_bonded_dem() {
    // create five particles,
    // set it up and check if the particles are bonded correctly
    let spacing = 1.0;
    let radius = spacing / 2.;
    let x = vec![-1., 0., 1., 0., 0.];
    let y = vec![0., 0., 0., 1., -1.];
    let mut beam = DemBonded::new(x.len(), 0, "beam".to_string());
    setup_particle_properties(&mut beam, x, y, radius);

    setup_bonded_structure(&mut beam, 1.2);

    /* particles are alligned the following way

    /             3

    /        0    1     2

    /             4

    So particle 0 is aligned with 1
    So particle 2 is aligned with 1
    So particle 3 is aligned with 1
    So particle 4 is aligned with 1
    So particle 1 is aligned with 0, 4, 2, 3

    Let's check that
     */

    println!("{:?}", beam.bonds);
    println!("{:?}", beam.bonds0);
    // particle 0
    assert_eq!(true, beam.bonds[0].contains_key(&1));
    assert_eq!(false, beam.bonds[0].contains_key(&2));

    // particle 1
    assert_eq!(true, beam.bonds[1].contains_key(&0));
    assert_eq!(false, beam.bonds[1].contains_key(&1));
    assert_eq!(true, beam.bonds[1].contains_key(&2));
    assert_eq!(true, beam.bonds[1].contains_key(&3));
    assert_eq!(true, beam.bonds[1].contains_key(&4));

    // particle 2
    assert_eq!(true, beam.bonds[2].contains_key(&1));
    assert_eq!(false, beam.bonds[2].contains_key(&4));

    // particle 3
    assert_eq!(true, beam.bonds[3].contains_key(&1));
    assert_eq!(false, beam.bonds[3].contains_key(&2));

    // particle 4
    assert_eq!(true, beam.bonds[4].contains_key(&1));
    assert_eq!(false, beam.bonds[4].contains_key(&3));
}


#[test]
fn test_tensile_force_on_bonded_dem() {
    // create two particles, on a single line and create a bonded dem entity.
    let spacing = 1.0;
    let radius = spacing / 2.;
    let x = vec![0., 1.0];
    let y = vec![0., 0.0, 0.0, 0.0];
    let mut beam = DemBonded::new(x.len(), 0, "beam".to_string());
    setup_particle_properties(&mut beam, x, y, radius);

    setup_bonded_structure(&mut beam, 1.2);

    // apply some force on
}
