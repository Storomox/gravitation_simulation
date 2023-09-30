use crate::Particle;

pub fn calc(particle1: &Particle, particle2: &Particle) -> f64 {
    const G: f64 = 6.6730e-11; 

    let velocity: f64 = (G * particle1.mass / particle1.radius_particle).sqrt();
    println!("v_1 = {}", velocity);

    velocity
}