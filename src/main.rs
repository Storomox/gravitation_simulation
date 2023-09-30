struct Particle {
    mass: f64,
    position: (f64, f64),
    radius_particle: f64,
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

    println!("M = {}, m = {}", particle1.mass, particle2.mass);
    println!("r = {}" ,distance);

    // Avoid divisions by zero return zero force if particles are at the same position
    if distance == 0.0 {
        return 0.0;
    } 
    
    // G * M * m / r^2 = Gravitationskraft
    let force_magnitude: f64 = (G * particle1.mass * particle2.mass) / (distance * distance);

    

    force_magnitude

    
}

fn kosmische_geschwindigkeiten(particle1: &Particle, particle2: &Particle) -> f64 {
    const G: f64 = 6.6730e-11; 

    let velocity: f64 = (G * particle1.mass / particle1.radius_particle).sqrt();
    println!("v_1 = {}", velocity);

    velocity
}

fn main() {
    
    let particle1: Particle = Particle {
        // Beispiel Sonne
        mass: 1.99e30, // Masse in Kg
        position: (0.0, 0.0), // Position (0 | 0) -> Ursprung
        radius_particle: 696342000.0, // Radius Sonne in m
    };

    
    let particle2: Particle = Particle {
        // Beispiel Erde
        mass: 5.97e24, // Masse in Kg
        position: (0.0, 149598000e3), // Abstand in Meter,
        radius_particle: 6371000.0, // Radius Erde in m
    };

   
    let force_magnitude: f64 = calculate_gravitational_force(&particle1, &particle2);
    kosmische_geschwindigkeiten(&particle1, &particle2);

    print!("Gravitationskraft: ({})", force_magnitude);
}
