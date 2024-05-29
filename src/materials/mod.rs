use nalgebra::{Point3, Vector3}; 
use crate::ray::Ray;
use crate::objects::Object;

pub mod opaque;
pub mod diffuse;
pub mod specular;

pub trait Material {
    fn color(&self) -> Vector3<u8>;

    fn reflectance(&self) -> f32;

    fn bounce(&self, ray: &Ray, obj: &Box<dyn Object>, pos: &Point3<f32>, intersection: &Point3<f32>) -> Ray;
}
