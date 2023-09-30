struct Particle {
    mass: f64,
    position: (f64, f64),
}


fn calculate_gravitational_force(particle1: &Particle, particle2: &Particle) -> f64 {
    // constante G, 6,6730*10^-11
    const G: f64 = 6.6730e-11;

    // calculate the displacement vector between the two particles
    
    let mut v: Vec<f64> = Vec::new();
    let dx: f64 = particle1.position.0 - particle2.position.0;
    let dy: f64 = particle1.position.1 - particle2.position.1;
    v.push(dx);
    v.push(dy);


    println!("vector = {:?}", &v[0]);

    // calculate the distance between the two particles (a^2 + b^2 = c^2)
    let distance: f64 = (&v[0] * &v[0] + &v[1] * &v[1]).sqrt();

    println!("r = {}" ,distance);

    // Avoid divisions by zero return zero force if particles are at the same position
    if distance == 0.0 {
        return 0.0;
    } 
    
    // G * M * m / r^2 = Gravitationskraft
    let force_magnitude: f64 = (G * particle1.mass * particle2.mass) / (distance * distance);

    let velocity: f64 = (G * particle2.mass / distance).sqrt();
    println!("v = {}", velocity);

    println!("Test");

    force_magnitude

    
}

fn main() {
    let particle2: Particle = Particle {
        mass: 1.0e12, // Masse in Kg
        position: (0.0, 0.0),
    };

    let particle1: Particle = Particle {
        mass: 2.0e12,
        position: (150.0, 150.0),
    };

    let force_magnitude: f64 = calculate_gravitational_force(&particle1, &particle2);

    print!("Gravitationskraft: ({})", force_magnitude);
}
