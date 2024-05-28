use nalgebra::{Vector3, Point3};
use crate::objects::Object;
use crate::ray::Ray;

pub struct Plane {
    snorm: Vector3<f32>,
}

impl Plane {
    pub fn new(snorm: Vector3<f32>) -> Self {
        Self {
            snorm,
        }
    }
}

impl Object for Plane {
    fn intersection_point(&self, pos: &Point3<f32>, ray: &Ray, tolerance: f32) -> Option<Point3<f32>> {
        let t = (pos - ray.pos()).dot(&self.snorm) / ray.dir().dot(&self.snorm);
        if t > tolerance {
            return Some(ray.at(t))
        }
        None
    }

    fn surface_normal(&self, _pos: &Point3<f32>, _pt: &Point3<f32>) -> Vector3<f32> {
        self.snorm
    }
}
