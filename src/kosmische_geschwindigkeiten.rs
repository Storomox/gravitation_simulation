use crate::Particle;

// TODO Rename fn name -> 1. Kosmische Geschwindigkeit
pub fn calc(particle1: &Particle, _particle2: &Particle) -> f64 {
    const G: f64 = 6.6730e-11; 

    let velocity: f64 = (G * particle1.mass / particle1.radius_particle).sqrt();
    println!("v_1 = {}", velocity);

    velocity
}

// TODO 2. Kosmische geschwindigkeit