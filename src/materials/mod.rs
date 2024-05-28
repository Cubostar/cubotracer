use nalgebra::{Point3, Vector3}; 
use rand::{Rng, thread_rng};
use crate::ray::Ray;
use crate::objects::Object;

pub mod opaque;
use opaque::Opaque;
pub mod diffuse;
use diffuse::Diffuse;
pub mod specular;
use specular::Specular;

pub trait Material {
    fn color(&self) -> Vector3<u8>;

    fn reflectance(&self) -> f32;

    fn bounce(&self, ray: &Ray, obj: &Box<dyn Object>, pos: &Point3<f32>, intersection: &Point3<f32>) -> Ray;
}

impl Material for Opaque {
    fn color(&self) -> Vector3<u8> {
        self.color()
    }

    fn reflectance(&self) -> f32 {
        0.0
    }

    fn bounce(&self, ray: &Ray, _obj: &Box<dyn Object>, _pos: &Point3<f32>, _intersection: &Point3<f32>) -> Ray {
        *ray
    }
}

impl Material for Diffuse {
    fn color(&self) -> Vector3<u8> {
        self.color()
    }

    fn reflectance(&self) -> f32 {
        self.reflectance()
    }

    fn bounce(&self, _ray: &Ray, obj: &Box<dyn Object>, pos: &Point3<f32>, intersection: &Point3<f32>) -> Ray {
        let bounce_dir: Vector3<f32> = (obj.surface_normal(pos, intersection) + random_unit_vector()).normalize();
        Ray::new(*intersection, bounce_dir)
    }
}

impl Material for Specular {
    fn color(&self) -> Vector3<u8> {
        self.color()
    }

    fn reflectance(&self) -> f32 {
        self.reflectance()
    }

    fn bounce(&self, ray: &Ray, obj: &Box<dyn Object>, pos: &Point3<f32>, intersection: &Point3<f32>) -> Ray {
        let n = obj.surface_normal(pos, intersection);
        Ray::new(*intersection, ray.dir() - (2.0 * n.dot(&ray.dir()) * n))
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
