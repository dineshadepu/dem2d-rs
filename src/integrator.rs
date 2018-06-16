use DemDiscrete;

pub fn integrate(src: &mut DemDiscrete, dt: f32){
    let entity = src;
    for i in 0..entity.len{
        entity.u[i] += entity.fx[i] * entity.m_inv[i] * dt;
        entity.v[i] += entity.fy[i] * entity.m_inv[i] * dt;
        entity.x[i] += entity.u[i] * dt;
        entity.y[i] += entity.v[i] * dt;
    }
}
