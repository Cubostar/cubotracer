use nalgebra::{Point3, Vector3};
use cubotracer::ppmhandler::{PPMHandler, PPMType};
use cubotracer::objects::mesh::Mesh;
use cubotracer::materials::diffuse::Diffuse;
use cubotracer::camera::Camera;
use cubotracer::world::World;
use cubotracer::ray::Ray;

fn background_color(ray: &Ray) -> Vector3<u8> {
    let a: f32 = 0.5 * (ray.dir().y + 1.0);
    let color = ((1.0 - a) * Vector3::<f32>::new(255.0, 255.0, 255.0)) + (a * Vector3::<f32>::new(127.0, 178.0, 255.0));
    Vector3::<u8>::new(color.x as u8, color.y as u8, color.z as u8)
}

fn main() {
    let mut world = World::new(Box::new(background_color));
    let max_bounces = 5;
    let rays_per_pixel = 5;

    let bunny = Mesh::from_obj("assets/bunny.obj".to_string()).expect("MeshError: Error parsing .obj file");

    world.add_camera(
        Camera::new(
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            400,
            0.873,
            16.0 / 9.0,
            3.4,
            0.0,
        ),
        Point3::new(0.0, 0.0, -0.7),
        "camera".to_string());
    world.add_object(Box::new(bunny), Point3::new(0.0, -0.1, -1.0), Box::new(Diffuse::new(Vector3::new(153, 50, 204), 0.5)), "bunny".to_string());

    let result = world.render("camera".to_string(), max_bounces, rays_per_pixel);

    let handler = PPMHandler();
    handler.write(&result, PPMType::P6, "output".to_string()).expect("Error writing rendered image");
}
