use crate::Particle;

pub fn calc(particle1: &Particle, particle2: &Particle) -> f64 {
    const G: f64 = 6.6730e-11; 

    let velocity: f64 = (2.0 * G * particle1.mass * ( 1.0 / particle1.radius_particle - 1.0 / 2.0 * particle2.position.0)).sqrt();
    println!("v_1 = {}", velocity);

    velocity
}