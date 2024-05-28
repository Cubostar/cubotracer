use nalgebra::{Vector3, Point3};

#[derive(Copy, Clone)]
pub struct Ray {
    pt: Point3<f32>,
    dir: Vector3<f32>,
}

impl Ray {
    pub fn new(pt: Point3<f32>, dir: Vector3<f32>) -> Self {
        Self {
            pt,
            dir: dir.normalize(),
        }
    }

    pub fn at(&self, t: f32) -> Point3<f32> {
        if t < 0.0 {
            panic!("Ray queried at negative time t = {}", t)
        }
        self.pt + (t * self.dir)
    }

    pub fn pos(&self) -> Point3<f32> {
        self.pt
    }

    pub fn dir(&self) -> Vector3<f32> {
        self.dir
    }
}

