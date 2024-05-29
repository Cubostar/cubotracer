use nalgebra::{Vector3, Point3};
use rand::{Rng, thread_rng};
use crate::materials::Material;
use crate::objects::Object;
use crate::ray::Ray;

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
}

impl Material for Diffuse {
    fn color(&self) -> Vector3<u8> {
        self.color
    }

    fn reflectance(&self) -> f32 {
        self.reflectance
    }

    fn bounce(&self, _ray: &Ray, obj: &Box<dyn Object>, pos: &Point3<f32>, intersection: &Point3<f32>) -> Ray {
        let bounce_dir: Vector3<f32> = (obj.surface_normal(pos, intersection) + random_unit_vector()).normalize();
        Ray::new(*intersection, bounce_dir)
    }
}

fn random_unit_vector() -> Vector3<f32> {
    let mut rng = thread_rng();
    let mut result = Vector3::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0));
    while result.magnitude() > 1.0 {
        result = Vector3::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0));
    }
    result.normalize()
}
