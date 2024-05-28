pub struct Sphere {
    radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
        }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}
