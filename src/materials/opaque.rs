use nalgebra::Vector3;

pub struct Opaque {
    color: Vector3<u8>
}

impl Opaque {
    pub fn new(color: Vector3<u8>) -> Self {
        Self {
            color,
        }
    }
    
    pub fn color(&self) -> Vector3<u8> {
        self.color
    }
}

