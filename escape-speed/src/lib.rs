/// Universal Gravitational Constant
const G: f64 = 6.67430e-11;

#[derive(Debug)]
pub struct HeavenlyObject {
    name: String,
    mass: f64,
    radius: f64,
}

impl HeavenlyObject {
    pub fn new(name: &str, mass: f64, radius: f64) -> Self {
        Self {
            name: String::from(name),
            mass,
            radius,
        }
    }
}

pub fn solar_system() -> Vec<HeavenlyObject> {
    vec![
        HeavenlyObject::new("Sun", 1_988_500e24, 695_700_f64),
        HeavenlyObject::new("Mercury", 0.33e24, 2_439.5_f64),
        HeavenlyObject::new("Venus", 4.87e24, 6_052_f64),
        HeavenlyObject::new("Earth", 5.97e24, 6_378_f64),
        HeavenlyObject::new("Mars", 0.642e24, 3_396_f64),
        HeavenlyObject::new("Jupiter", 1898e24, 71_492_f64),
        HeavenlyObject::new("Saturn", 568e24, 60_268_f64),
        HeavenlyObject::new("Uranus", 86.8e24, 25_559_f64),
        HeavenlyObject::new("Neptune", 102e24, 24_764_f64),
    ]
}

pub trait HeavenlyBody {
    fn mass(&self) -> f64;
    fn average_radius(&self) -> f64;
    fn escape_speed(&self, distance: f64) -> f64;
}

impl HeavenlyBody for HeavenlyObject {
    fn mass(&self) -> f64 {
        self.mass
    }

    fn average_radius(&self) -> f64 {
        self.radius
    }

    fn escape_speed(&self, distance: f64) -> f64 {
        f64::sqrt((G * self.mass) / (self.radius + distance))
    }
}
