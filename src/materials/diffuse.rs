use nalgebra::Vector3;

pub struct Diffuse {
    color: Vector3<u8>,
    reflectance: f32,
}

impl Diffuse {
    pub fn new(color: Vector3<u8>, reflectance: f32) -> Self {
        if reflectance <= 0.0 || 1.0 <= reflectance {
            panic!("Reflectance must be a number between 0 and 1, given reflectance was {}", reflectance)
        }
        Self {
            color,
            reflectance,
        }
    }

    pub fn color(&self) -> Vector3<u8> {
        self.color
    }

    pub fn reflectance(&self) -> f32 {
        self.reflectance
    }
}

