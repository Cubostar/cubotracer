use nalgebra::{Vector3, Point3};
use std::io::{BufReader, BufRead};
use std::fs::File;
use crate::objects::Object;
use crate::ray::Ray;

pub mod triangle;
use triangle::Triangle;

pub struct Mesh {
    triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn from_obj(path: String) -> std::io::Result<Self> {
        let reader = BufReader::new(File::open(path)?);
        let lines = reader.lines();
        let mut vertices: Vec<Vector3<f32>> = vec![];
        let mut triangles: Vec<Triangle> = vec![];
        for line_result in lines {
            if let Ok(line) = line_result {
                if line.starts_with("#") {
                    continue;
                } else {
                   Self::read_line(&line, &mut vertices, &mut triangles); 
                }
            }
        }
        Ok(Self {
            triangles,
        })
    }

    fn read_line(line: &str, vertices: &mut Vec<Vector3<f32>>, triangles: &mut Vec<Triangle>) -> () {
        if line.starts_with("#") {
        } else if line.starts_with("v") {
            let mut coords = line.split_whitespace();
            coords.next();
            vertices.push(Vector3::<f32>::new(
                coords.next().unwrap().parse::<f32>().unwrap(),
                coords.next().unwrap().parse::<f32>().unwrap(),
                coords.next().unwrap().parse::<f32>().unwrap()));
        } else if line.starts_with("f") {
            let mut indices = line.split_whitespace();
            indices.next();
            triangles.push(Triangle::new(
                vertices[indices.next().unwrap().parse::<usize>().unwrap() - 1],
                vertices[indices.next().unwrap().parse::<usize>().unwrap() - 1],
                vertices[indices.next().unwrap().parse::<usize>().unwrap() - 1]));
        }
    }
}

impl Object for Mesh {
    fn surface_normal(&self, pos: &Point3<f32>, pt: &Point3<f32>) -> Vector3<f32> {
        let mut closest = (self.triangles.get(0).expect("MeshError: No triangles in mesh"), self.triangles.get(0).expect("MeshError: No triangles in mesh").distance_to_point(pos, pt));
        for tri in self.triangles.iter() {
            let tri_distance = tri.distance_to_point(pos, pt);
            if tri_distance < closest.1 {
                closest = (tri, tri_distance);
            }
        }
        closest.0.surface_normal(pos, pt)
    }

    fn intersection_point(&self, pos: &Point3<f32>, ray: &Ray, tolerance: f32) -> Option<Point3<f32>> {
        let mut closest: Option<Point3<f32>> = None;
        let intersections: Vec<Option<Point3<f32>>> = self.triangles.iter().map(|tri| tri.intersection_point(pos, ray, tolerance)).collect();
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
