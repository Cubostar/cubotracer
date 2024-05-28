use nalgebra::{Point3, Vector3};

pub struct Triangle { // Points are in counter-clockwise winding order
    v1: Vector3<f32>,
    v2: Vector3<f32>,
    v3: Vector3<f32>,
    snorm: Vector3<f32>,
}

impl Triangle {
    pub fn new(v1: Vector3<f32>, v2: Vector3<f32>, v3: Vector3<f32>) -> Self {
        Self {
            v1,
            v2,
            v3,
            snorm: (v1 - v2).cross(&(v2 - v3)).normalize(),
        }
    }

    pub fn v1(&self) -> Vector3<f32> {
        self.v1
    }

    pub fn v2(&self) -> Vector3<f32> {
        self.v2
    }

    pub fn v3(&self) -> Vector3<f32> {
        self.v3
    }

    pub fn snorm(&self) -> Vector3<f32> {
        self.snorm
    }

    pub fn distance_to_point(&self, pos: &Point3<f32>, point: &Point3<f32>) -> f32 {
        let p1 = pos + self.v1;
        let p2 = pos + self.v2;
        let p3 = pos + self.v3;
        let point_to_plane = (point - p1).dot(&self.snorm);
        let projected = point - (point_to_plane * self.snorm);
        if self.snorm.dot(&(p2 - p1).cross(&(projected - p1))) > 0.0 && 
        self.snorm.dot(&(p3 - p2).cross(&(projected - p2))) > 0.0 && 
        self.snorm.dot(&(p1 - p3).cross(&(projected - p3))) > 0.0 {
            return point_to_plane
        }
        distance_from_point_to_line(point, &p1, &p2).min(distance_from_point_to_line(point, &p2, &p3).min(distance_from_point_to_line(point, &p3, &p1)))
    }
}

fn distance_from_point_to_line(p: &Point3<f32>, a: &Point3<f32>, b: &Point3<f32>) -> f32 {
    if (b - a).dot(&(p - a)) <= 0.0 {
        return (p - a).magnitude()
    } else if (p - b).dot(&(b - a)) >= 0.0 {
        return (p - b).magnitude()
    }
    (b - a).cross(&(p - a)).magnitude() / (b - a).magnitude()
}
