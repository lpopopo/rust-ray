pub struct Vec3 {
    e: (f64, f64, f64),
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: (e0, e1, e2) }
    }

    pub fn x(&self) -> f64 {
        self.e.0
    }

    pub fn y(&self) -> f64 {
        self.e.1
    }

    pub fn z(&self) -> f64 {
        self.e.2
    }
}
