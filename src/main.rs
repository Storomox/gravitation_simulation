mod grav_force;
mod geschwindigkeiten;
pub struct Particle {
    mass: f64,
    position: (f64, f64),
    radius_particle: f64,
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

   
    let force_magnitude: f64 = grav_force::calculate_gravitational_force(&particle1, &particle2);
    geschwindigkeiten::calc(&particle1, &particle2);

    print!("Gravitationskraft: ({})", force_magnitude);
}
