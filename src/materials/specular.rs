use nalgebra::{Vector3, Point3};
use crate::materials::Material;
use crate::objects::Object;
use crate::ray::Ray;

pub struct Specular {
    color: Vector3<u8>,
    reflectance: f32,
}

impl Specular {
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

impl Material for Specular {
    fn color(&self) -> Vector3<u8> {
        self.color
    }

    fn reflectance(&self) -> f32 {
        self.reflectance
    }

    fn bounce(&self, ray: &Ray, obj: &Box<dyn Object>, pos: &Point3<f32>, intersection: &Point3<f32>) -> Ray {
        let n = obj.surface_normal(pos, intersection);
        Ray::new(*intersection, ray.dir() - (2.0 * n.dot(&ray.dir()) * n))
    }
}
