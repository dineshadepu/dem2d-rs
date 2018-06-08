use super::DemDiscrete;
use base::Base;

fn body_force<T: Base>(entity: &mut T, gx: f32, gy: f32){
    let mut fx = entity.get_mut_fx();
    let mut fy = entity.get_mut_fy();
    let m = &mut entity.get_m();
    for i in 0..*entity.get_len(){
        fx[i] += m[i] * gx;
        fy[i] += m[i] * gy;
    }
}
