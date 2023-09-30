use crate::Particle;
mod kosmische_geschwindigkeiten;

impl Particle {
    fn calc_exentrizitaet(particle1: &Particle, particle2: Particle) -> (f64, f64) {
        // B_1 = (75 | 0); P_2 = (100 | 0);

        // Rechne Punkte aus, speichern im Vektor
        let mut v: Vec<f64> = Vec::new();
        let m: f64 = 0.0; // Masse Punkt
        let e: f64 = particle1.position.0; // Perihel, Aphel
        let a_start: f64 = particle2.position.0; // !TODO Allgemeine Formel, nicht dynamisch...

        let v_1 =  kosmische_geschwindigkeiten::calc(particle1.mass, particle2.mass); // 1. Kosmische..., wir brauchen aber die zweiter

        let b_extrem: f64 = (a * a - e * e).sqrt(); // Nur der extem Wert
    }
}