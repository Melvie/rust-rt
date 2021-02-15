use rand::Rng;
use rust_rt::camera::Camera;
use rust_rt::objects::{Hit, Object, SceneObjects, Sphere};
use rust_rt::ray::Ray;
use rust_rt::vec3::{Colour, Point3D};

pub fn ray_colour(ray: &Ray, world: &SceneObjects) -> Colour {
    if let Some(hit_record) = world.hit(ray, 0.0, std::f64::INFINITY) {
        return 0.5 * (hit_record.normal() + Colour::new(1.0, 1.0, 1.0));
    }
    let unit_direction = ray.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_WIDTH: i32 = 400;
    const IMG_HEIGHT: i32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i16 = 100;
    const MAX_DEPTH: i16 = 50;

    let mut world = SceneObjects::new();


    world.add(Object::Sphere(Sphere::new(
        Point3D::new(0.0, 0.0, -1.0),
        0.5,
    )));


    world.add(Object::Sphere(Sphere::new(
        Point3D::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let camera = Camera::new(ASPECT_RATIO, 2.0, 1.0);

    println!("P3\n{:?} {:?}\n255", IMG_WIDTH, IMG_HEIGHT);
    let mut rng = rand::thread_rng();

    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("Scanning lines remaining:{:?}", j);
        for i in 0..IMG_WIDTH {
            let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
            for _x in 0..SAMPLES_PER_PIXEL {
                let u: f64 = (i as f64 + rng.gen::<f64>()) / (IMG_WIDTH - 1) as f64;
                let v: f64 = (j as f64 + rng.gen::<f64>()) / (IMG_HEIGHT - 1) as f64;

                let ray = camera.get_ray(u, v);
                pixel_colour += ray_colour(&ray, &world);
            }
            println!("{:}", pixel_colour.write_colour(SAMPLES_PER_PIXEL));
        }
        eprint!("{}[2J", 27 as char);
    }
}
