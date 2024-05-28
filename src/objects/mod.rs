use nalgebra::{Point3, Vector3};
use crate::ray::Ray;

pub mod sphere;
use sphere::Sphere;
pub mod mesh;
use mesh::Mesh;
use mesh::triangle::Triangle;

pub trait Object {
    // Returns closest intersection point if there is one (otherwise returns None)
    fn intersection_point(&self, pos: &Point3<f32>, ray: &Ray, tolerance: f32) -> Option<Point3<f32>>;

    fn surface_normal(&self, pos: &Point3<f32>, pt: &Point3<f32>) -> Vector3<f32>;
}

impl Object for Sphere {
    fn intersection_point(&self, center: &Point3<f32>, ray: &Ray, tolerance: f32) -> Option<Point3<f32>> {
        let a = ray.dir().dot(&ray.dir());
        let b = 2.0 * ray.dir().dot(&(ray.pos() - center));
        let c = (ray.pos() - center).dot(&(ray.pos() - center)) - self.radius().powi(2);
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

impl Object for Triangle {
    fn intersection_point(&self, pos: &Point3<f32>, ray: &Ray, tolerance: f32) -> Option<Point3<f32>> {
        let p1 = pos + self.v1();
        let p2 = pos + self.v2();
        let p3 = pos + self.v3();
        if self.snorm().dot(&ray.dir()) == 0.0 {
            return None
        }
        let d = -self.snorm().dot(&p1.coords); 
        let t = -(self.snorm().dot(&ray.pos().coords) + d) / (self.snorm().dot(&ray.dir()));
        if t <= tolerance {
            return None
        }
        let p = ray.at(t);
        if self.snorm().dot(&(p2 - p1).cross(&(p - p1))) > 0.0 && 
        self.snorm().dot(&(p3 - p2).cross(&(p - p2))) > 0.0 && 
        self.snorm().dot(&(p1 - p3).cross(&(p - p3))) > 0.0 {
            return Some(p)
        }
        None
    }

    fn surface_normal(&self, _pos: &Point3<f32>, _pt: &Point3<f32>) -> Vector3<f32> {
        self.snorm()
    }
}

impl Object for Mesh {
    fn surface_normal(&self, pos: &Point3<f32>, pt: &Point3<f32>) -> Vector3<f32> {
        let mut closest = (self.triangles().get(0).expect("MeshError: No triangles in mesh"), self.triangles().get(0).expect("MeshError: No triangles in mesh").distance_to_point(pos, pt));
        for tri in self.triangles().iter() {
            let tri_distance = tri.distance_to_point(pos, pt);
            if tri_distance < closest.1 {
                closest = (tri, tri_distance);
            }
        }
        closest.0.surface_normal(pos, pt)
    }

    fn intersection_point(&self, pos: &Point3<f32>, ray: &Ray, tolerance: f32) -> Option<Point3<f32>> {
        let mut closest: Option<Point3<f32>> = None;
        let intersections: Vec<Option<Point3<f32>>> = self.triangles().iter().map(|tri| tri.intersection_point(pos, ray, tolerance)).collect();
        for intersection in intersections.iter() {
            match intersection {
                Some(point) => {
                    match closest {
                        Some(prev) => {
                            if (ray.pos() - point).magnitude() < (ray.pos() - prev).magnitude() {
                                closest = Some(*point);
                            }
                        }
                        None => closest = Some(*point),
                    }
                },
                None => { },
            }
        }
        closest
    }
}
