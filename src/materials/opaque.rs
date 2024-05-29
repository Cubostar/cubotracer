use nalgebra::{Vector3, Point3};
use crate::materials::Material;
use crate::objects::Object;
use crate::ray::Ray;

pub struct Opaque {
    color: Vector3<u8>
}

impl Opaque {
    pub fn new(color: Vector3<u8>) -> Self {
        Self {
            color,
        }
    }
}

impl Material for Opaque {
    fn color(&self) -> Vector3<u8> {
        self.color
    }

    fn reflectance(&self) -> f32 {
        0.0
    }

    fn bounce(&self, ray: &Ray, _obj: &Box<dyn Object>, _pos: &Point3<f32>, _intersection: &Point3<f32>) -> Ray {
        *ray
    }
}
