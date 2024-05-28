use nalgebra::{Point3, Vector3};
use crate::ray::Ray;

pub mod sphere;
pub mod mesh;
pub mod plane;

pub trait Object {
    // Returns closest intersection point if there is one (otherwise returns None)
    fn intersection_point(&self, pos: &Point3<f32>, ray: &Ray, tolerance: f32) -> Option<Point3<f32>>;

    fn surface_normal(&self, pos: &Point3<f32>, pt: &Point3<f32>) -> Vector3<f32>;
}
