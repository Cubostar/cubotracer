use nalgebra::{Vector3, Point3};
use crate::objects::Object;
use crate::ray::Ray;

pub struct Sphere {
    radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
        }
    }
}

impl Object for Sphere {
    fn intersection_point(&self, center: &Point3<f32>, ray: &Ray, tolerance: f32) -> Option<Point3<f32>> {
        let a = ray.dir().dot(&ray.dir());
        let b = 2.0 * ray.dir().dot(&(ray.pos() - center));
        let c = (ray.pos() - center).dot(&(ray.pos() - center)) - self.radius.powi(2);
        let d = b.powi(2) - (4.0 * a * c);
        if d >= 0.0 {
            let t = ((-b + d.sqrt()) / (2.0 * a)).min((-b - d.sqrt()) / (2.0 * a));
            if t > tolerance {
                return Some(ray.at(t))
            }
        }
        None
    }

    fn surface_normal(&self, pos: &Point3<f32>, pt: &Point3<f32>) -> Vector3<f32> {
        (pt - pos).normalize() 
    }
}
