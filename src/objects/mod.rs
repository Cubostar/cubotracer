use nalgebra::{Point3, Vector3};
use crate::ray::Ray;

pub mod sphere;
pub mod mesh;

pub trait Object {
    // Returns closest intersection point if there is one (otherwise returns None)
    fn intersection_point(&self, pos: &Point3<f32>, ray: &Ray, tolerance: f32) -> Option<Point3<f32>>;

    fn surface_normal(&self, pos: &Point3<f32>, pt: &Point3<f32>) -> Vector3<f32>;
}

pub struct Ground {
}

impl Ground {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Object for Ground {
    fn intersection_point(&self, pos: &Point3<f32>, ray: &Ray, tolerance: f32) -> Option<Point3<f32>> {
        let t = (pos.y - ray.pos().y) / ray.dir().y;
        if t > tolerance {
            return Some(ray.at(t))
        }
        None
    }

    fn surface_normal(&self, _pos: &Point3<f32>, _pt: &Point3<f32>) -> Vector3<f32> {
        Vector3::new(0.0, 1.0, 0.0).normalize()
    }
}
