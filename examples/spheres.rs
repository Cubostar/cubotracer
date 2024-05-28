use nalgebra::{Point3, Vector3};
use cubotracer::ppmhandler::{PPMHandler, PPMType};
use cubotracer::materials::{opaque::Opaque, diffuse::Diffuse, specular::Specular};
use cubotracer::camera::Camera;
use cubotracer::world::World;
use cubotracer::objects::sphere::Sphere;
use cubotracer::ray::Ray;

fn background_color(ray: &Ray) -> Vector3<u8> {
    let a: f32 = 0.5 * (ray.dir().y + 1.0);
    let color = ((1.0 - a) * Vector3::<f32>::new(255.0, 255.0, 255.0)) + (a * Vector3::<f32>::new(127.0, 178.0, 255.0));
    Vector3::<u8>::new(color.x as u8, color.y as u8, color.z as u8)
}

fn main() {
    let mut world = World::new(Box::new(background_color));

    let opaque_sphere = Sphere::new(0.5);
    let diffuse_sphere = Sphere::new(0.6);
    let specular_sphere = Sphere::new(0.5);

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
        Point3::new(0.0, 0.1, -0.5),
        "camera".to_string());
    world.add_object(Box::new(diffuse_sphere), Point3::new(0.7, -0.3, -2.9), Box::new(Diffuse::new(Vector3::<u8>::new(179, 77, 77), 0.5)), "dsphere".to_string());
    world.add_object(Box::new(specular_sphere), Point3::new(0.0, 0.6, -3.0), Box::new(Specular::new(Vector3::<u8>::new(204, 204, 204), 0.5)), "ssphere".to_string());
    world.add_object(Box::new(opaque_sphere), Point3::new(-0.7, -0.3, -3.0), Box::new(Opaque::new(Vector3::<u8>::new(153, 50, 204))), "osphere".to_string());

    let result = world.render("camera".to_string(), 10, 5);
    println!("Render finished");

    let handler = PPMHandler();
    handler.write(&result, PPMType::P6, "output".to_string()).expect("Error writing rendered image");
}
