use crate::Particle;
mod geschwindigkeiten;

impl Particle {
    fn calc(particle1: &Particle, particle2: Particle) -> (f64, f64) {
        // B_1 = (75 | 0); P_2 = (100 | 0);

        // Rechne Punkte aus, speichern im Vektor
        // let mut v: Vec<f64> = Vec::new();
        // let m: f64 = 0.0; // Masse Punkt
        // let e: f64 = particle1.position.0; // Perihel, Aphel
        // let a_start: f64 = particle2.position.0; // TODO Allgemeine Formel, nicht dynamisch...



        let v_1 =  geschwindigkeiten::calc(particle1.mass, particle2.mass);

        let b_extrem: f64 = (a * a - e * e).sqrt(); // Nur der extem Wert
    }
}