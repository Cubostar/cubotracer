use rand::Rng;
use rand::distributions::Uniform;
use crate::math::{Ray, random_vector_on_unit_disk};
use na::{Vector3, Point3, UnitQuaternion};

pub struct Camera {
    up: Vector3<f32>,
    right: Vector3<f32>,
    dir: Vector3<f32>,
    iwidth: usize,
    iheight: usize,
    vheight: f32,
    vwidth: f32,
    fdist: f32,
    defangle: f32,
}

impl Camera {
    pub fn new(up: Vector3<f32>, right: Vector3<f32>, iwidth: usize, vfov: f32, aratio: f32, fdist: f32, defangle: f32) -> Self {
        let iheight: usize = if iwidth as f32 / aratio < 1.0 { 1 } else { (iwidth as f32 / aratio) as usize };
        let vheight: f32 = 2.0 * (vfov / 2.0).tan() * fdist;
        Self {
            up: up.normalize(),
            right: right.normalize(),
            dir: up.cross(&right).normalize(),
            iwidth,
            iheight,
            vheight,
            vwidth: vheight * (iwidth as f32 / iheight as f32), 
            fdist,
            defangle,
        }
    }

    pub fn get_iwidth(&self) -> usize {
        self.iwidth
    }

    pub fn get_iheight(&self) -> usize {
        self.iheight
    }

    pub fn get_rays(&self, pos: &Point3<f32>, rays_per_pixel: u32) -> Vec<(Ray, usize, usize)> {
        self.pixel_samples(pos, rays_per_pixel).iter().zip(self.ray_origins(pos, rays_per_pixel).iter()).map(|((psample, x, y), ori)| (Ray::new(*ori, psample - ori), *x, *y)).collect()
        //self.pixel_samples(pos, rays_per_pixel).iter().map(|(psample, x, y)| (Ray::new(*pos, psample - pos), *x, *y)).collect()
    }
    
    // Depth of Field Requirement
    fn ray_origins(&self, pos: &Point3<f32>, rays_per_pixel: u32) -> Vec<Point3<f32>> {
        let mut pixels: Vec<(usize, usize)> = vec![];
        for x in 0..self.iwidth {
            for y in 0..self.iheight {
                for _ in 0..rays_per_pixel {
                    pixels.push((x, y));
                }
            }
        }
        let defradius = self.fdist * (self.defangle / 2.0).tan();
        pixels.iter().map(
            |(_, _)|
            pos + (defradius * random_vector_on_unit_disk(&self.up, &self.right))
        ).collect()
    }

    // Antialiasing Requirement
    fn pixel_samples(&self, pos: &Point3<f32>, rays_per_pixel: u32) -> Vec<(Point3<f32>, usize, usize)> {
        let mut rng = rand::thread_rng();
        let interval = Uniform::new(-0.5, 0.5);
        let pwidth = self.vwidth / self.iwidth as f32;
        let pheight = self.vheight / self.iheight as f32;
        self.pixel_centers(pos).iter().map(
            |(c, x, y)|
            vec![1; rays_per_pixel as usize].iter().map(
                |_|
                (c + (self.up * pheight * rng.sample(interval)) + (self.right * pwidth * rng.sample(interval)), *x, *y)
            ).collect::<Vec<(Point3<f32>, usize, usize)>>()).flatten().collect::<Vec<(Point3<f32>, usize, usize)>>()
    }

    fn pixel_centers(&self, pos: &Point3<f32>) -> Vec<(Point3<f32>, usize, usize)> {
        let mut pixels: Vec<(usize, usize)> = vec![];
        for x in 0..self.iwidth {
            for y in 0..self.iheight {
                pixels.push((x, y));
            }
        }
        let image_topleft = pos + (self.fdist * self.dir) + (self.up * self.vheight / 2.0) - (self.right * self.vwidth / 2.0);
        let pwidth = self.vwidth / self.iwidth as f32;
        let pheight = self.vheight / self.iheight as f32;
        pixels.iter().map(
            |(x, y)|
            (image_topleft + (*y as f32 * pheight * -self.up) + (*x as f32 * pwidth * self.right) + (pheight * -self.up / 2.0) + (pwidth * self.right / 2.0), *x, *y))
            .collect()
    }
 
    // Camera Requirement
    // TODO: make work for when camera rotates between looking up and looking down
    pub fn lookat(&mut self, target: Point3<f32>, pos: Point3<f32>) -> () {
        let new_dir = (target - pos).normalize();
        // Quaternions for top and bottom view
        if new_dir.dot(&Vector3::new(0.0, 1.0, 0.0)).abs() == 1.0 {
            let rotation = UnitQuaternion::rotation_between(&self.dir, &new_dir);
            match rotation {
                Some(rot) => {
                    self.dir = rot * self.dir;
                    self.right = rot * self.right;
                    self.up = rot * self.up;
                },
                None => panic!("Attempted to rotate camera between looking up and looking down")
            }
        // Otherwise uses traditional lookat 
        } else {
            self.dir = new_dir;
            self.right = self.dir.cross(&Vector3::new(0.0, 1.0, 0.0));
            self.up = self.right.cross(&self.dir);
        }
    }
}
