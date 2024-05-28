use nalgebra::Vector3;
use std::io::{BufReader, BufRead};
use std::fs::File;

pub mod triangle;
use triangle::Triangle;

pub struct Mesh {
    triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }

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

