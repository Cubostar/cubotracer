use crate::math::{Ray, random_unit_vector};
use crate::objects::Object;
use na::{Point3, Vector3}; 

pub trait Material {
    fn color(&self) -> Vector3<u8>;

    fn reflectance(&self) -> f32;

    fn bounce(&self, ray: &Ray, obj: &Box<dyn Object>, pos: &Point3<f32>, intersection: &Point3<f32>) -> Ray;
}

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

pub struct Metal {
    color: Vector3<u8>,
    reflectance: f32,
}

impl Metal {
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

impl Material for Metal {
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
