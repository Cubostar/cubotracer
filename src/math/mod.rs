use nalgebra::{Vector3, Point3};
use rand::{Rng, thread_rng};

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

pub fn random_unit_vector() -> Vector3<f32> {
    let mut rng = thread_rng();
    let mut result = Vector3::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0));
    while result.magnitude() > 1.0 {
        result = Vector3::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0));
    }
    result.normalize()
}

pub fn random_vector_on_unit_disk(up: &Vector3<f32>, right: &Vector3<f32>) -> Vector3<f32> {
    let mut rng = thread_rng();
    let mut result: (f32, f32) = (rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0));
    while result.0.powi(2) + result.1.powi(2) > 1.0 {
        result = (rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0));
    }
    (result.0 * up) + (result.1 * right)
}

pub fn distance_from_point_to_line(p: &Point3<f32>, a: &Point3<f32>, b: &Point3<f32>) -> f32 {
    if (b - a).dot(&(p - a)) <= 0.0 {
        return (p - a).magnitude()
    } else if (p - b).dot(&(b - a)) >= 0.0 {
        return (p - b).magnitude()
    }
    (b - a).cross(&(p - a)).magnitude() / (b - a).magnitude()
}
