use nalgebra::{Point3, Vector3};
use crate::camera::Camera;
use crate::objects::Object;
use crate::ppmhandler::PPMImage;
use crate::ray::Ray;
use crate::materials::Material;
use std::collections::HashMap;

const TOL: f32 = 0.001; // TODO: maybe abstract or something?

#[kernel]
pub unsafe fn cuda_ray_color(ray: &[((f32, f32), (f32, f32))], num_bounces: u8, max_bounces: u8, colors: *mut [(u8, u8, u8)]) {
    let id = thread::index_1d()
    if num_bounces >= max_bounces {
        colors[id] += (0, 0, 0);
    }
    let intersection = self.intersection(ray);
    match intersection {
        Some((point, key)) => {
            let entry = self.objects.get(&key);
            match entry {
                Some((pos, obj, mat)) => {
                    let bounce = mat.bounce(ray, obj, pos, &point);
                    colors[id] += ((1.0 - mat.reflectance()) * mat.color().cast::<f32>()) 
                },
                None => panic!("Object key not found while computing ray color"),
            }
        },
        None => colors[id] += (0, 0, 0),
    }
}

pub struct World {
    cameras: HashMap<String, (Point3<f32>, Camera)>,
    objects: HashMap<String, (Point3<f32>, Box<dyn Object>, Box<dyn Material>)>,
    background_color: Box<dyn Fn(&Ray) -> Vector3<u8>>,
}

impl World {
    pub fn new(background_color: Box<dyn Fn(&Ray) -> Vector3<u8>>) -> Self {
        Self {
            cameras: HashMap::new(),
            objects: HashMap::new(),
            background_color,
        }
    }

    pub fn add_camera(&mut self, camera: Camera, pos: Point3<f32>, key: String) {
        self.cameras.insert(key, (pos, camera));
    }

    pub fn add_object(&mut self, object: Box<dyn Object>, pos: Point3<f32>, material: Box<dyn Material>, key: String) {
        self.objects.insert(key, (pos, object, material));
    }

    pub fn render(&self, key: String, max_bounces: u8, rays_per_pixel: u32) -> PPMImage {
        let entry = self.cameras.get(&key);
        match entry {
            Some((pos, camera)) => {
                let colors: Vec<(Vector3<u8>, usize, usize)> = camera.get_rays(pos, rays_per_pixel)
                    .iter()
                    .map(|(ray, x, y)| (self.ray_color(ray, 0, max_bounces), *x, *y))
                    .collect();
                let mut image: PPMImage = PPMImage::new(camera.get_iwidth() as usize, camera.get_iheight() as usize, 255);
                let mut counts: Vec<Vec<u32>> = vec![vec![0; camera.get_iheight()]; camera.get_iwidth()];
                for (color, x, y) in colors.iter() {
                    if counts[*x][*y] == 0 {
                        image.change_pixel(*x, *y, color.try_cast::<u32>().unwrap());
                    } else {
                        image.change_pixel(*x, *y, (color.try_cast::<u32>().unwrap() + (image.pixel_at(*x, *y) * counts[*x][*y])) / (counts[*x][*y] + 1));
                    }
                    counts[*x][*y] += 1;
                }
                image
            },
            None => panic!("Camera not found in world"),
        }
    }

    fn ray_color(&self, ray: &Ray, num_bounces: u8, max_bounces: u8) -> Vector3<u8> {
        if num_bounces >= max_bounces {
            return (self.background_color)(ray)
        }
        let intersection = self.intersection(ray);
        match intersection {
            Some((point, key)) => {
                let entry = self.objects.get(&key);
                match entry {
                    Some((pos, obj, mat)) => {
                        let bounce = mat.bounce(ray, obj, pos, &point);
                        (((1.0 - mat.reflectance()) * mat.color().cast::<f32>()) + 
                        ((self.ray_color(&bounce, num_bounces + 1, max_bounces).cast::<f32>() * mat.reflectance())))
                        .try_cast::<u8>().unwrap()
                    },
                    None => panic!("Object key not found while computing ray color"),
                }
            },
            None => (self.background_color)(ray),
        }
    }

    fn intersection(&self, ray: &Ray) -> Option<(Point3<f32>, String)> {
        let mut intersections: Vec<(Point3<f32>, String)> = vec![];
        for key in self.objects.keys() {
            let entry = self.objects.get(key);
            match entry {
                Some((pos, obj, _)) => {
                    let obj_intersection = obj.intersection_point(pos, ray, TOL);
                    match obj_intersection {
                        Some(point) => intersections.push((point, key.to_string())),
                        None => { },
                    }
                },
                None => { },
            }
        }
        if intersections.len() == 0 {
            None
        } else {
            let mut closest: (Point3<f32>, String) = intersections[0].clone();
            for intersection in intersections {
                if (ray.pos() - intersection.0).magnitude() < (ray.pos() - closest.0).magnitude() {
                    closest = intersection;
                }
            }
            Some(closest)
        }
    }

    // Camera Requirement
    pub fn move_camera_to(&mut self, new_pos: Point3<f32>, key: String) -> () {
        if let Some((pos, _)) = self.cameras.get_mut(&key) {
            *pos = new_pos;
        } else {
            panic!("Camera key not found while moving camera");
        }
    }

    pub fn make_camera_lookat(&mut self, target: Point3<f32>, key: String) -> () {
        if let Some((pos, camera)) = self.cameras.get_mut(&key) {
            camera.lookat(target, *pos);
        } else {
            panic!("Camera key not found while moving camera");
        }
    }
}
