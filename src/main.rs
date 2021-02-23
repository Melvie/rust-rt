use indicatif::{ParallelProgressIterator, ProgressBar};
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use rust_rt::camera::Camera;
use rust_rt::material::{Dielectric, Lambertian, Materials, Metal};
use rust_rt::objects::{Object, SceneObjects, Sphere};
use rust_rt::scene::Scene;
use rust_rt::vec3::{Colour, Point3D, Transpose, Vec3};
use rand::Rng;

fn write_render(rendered_scene: &Vec<Colour>, samples_per_pxl: i16) {
    for render in rendered_scene {
        println!("{:}", render.write_colour(samples_per_pxl));
    }
}

fn random_scene() -> SceneObjects {

    let mut world = SceneObjects::new();
    let material_ground = Materials::Lambertian(Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    world.add(Object::Sphere(Sphere::new(
        Point3D::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    let mut rng = rand::thread_rng();

    let ref_point = Point3D::new(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {

            let choose_mat = rng.gen::<f64>();

            let center = Point3D::new(a as f64 + 0.9*rng.gen::<f64>(), 0.2,  b as f64 +0.9*rng.gen::<f64>());

            if (center - ref_point).length() > 0.9 {
                let mat = if choose_mat <  0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    Materials::Lambertian(Lambertian::new(albedo))
                }
                else if choose_mat < 0.95 {
                    let albedo = Vec3::random_from_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);

                    Materials::Metal(Metal::new(albedo, fuzz))
                }
                else {
                    Materials::Dielectric(Dielectric::new(1.5))
                };

                world.add(Object::Sphere(Sphere::new(
                    center,
                    0.2,
                    mat,
                )));
            }
        }
    }

    let mat = Materials::Dielectric(Dielectric::new(1.5));
    world.add(Object::Sphere(Sphere::new(
        Point3D::new(0.0, 1.0, 0.0),
        1.0,
        mat,
    )));

    let mat2 = Materials::Lambertian(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    world.add(Object::Sphere(Sphere::new(
        Point3D::new(-4.0, 1.0, 0.0),
        1.0,
        mat2,
    )));

    let mat2 = Materials::Metal(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.add(Object::Sphere(Sphere::new(
        Point3D::new(4.0, 1.0, 0.0),
        1.0,
        mat2,
    )));


    world

}

fn main() {
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMG_WIDTH: i32 = 1200;
    const IMG_HEIGHT: i32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i16 = 500;
    const MAX_DEPTH: i16 = 50;

    let world = random_scene();

    let camera_origin = Point3D::new(13.0, 2.0, 3.0);
    let camera_target = Point3D::new(0.0, 0.0, 0.0);
    let focus_dist = 10.0;

    let scene = Scene::new(
        world,
        Camera::new(
            ASPECT_RATIO,
            20.0,
            camera_origin,
            camera_target,
            Point3D::new(0.0, 1.0, 0.0),
            0.1,
            focus_dist,
        ),
    );

    let bar = ProgressBar::new(SAMPLES_PER_PIXEL as u64);

    let rendered_scene: Vec<Vec3<f64>> = (0..SAMPLES_PER_PIXEL)
        .into_par_iter()
        .progress_with(bar)
        .map(|_| scene.render(MAX_DEPTH, IMG_WIDTH, IMG_HEIGHT))
        .collect::<Vec<Vec<Colour>>>()
        .transpose()
        .into_iter()
        .map(|x| x.into_iter().sum::<Colour>())
        .collect();

    println!("P3\n{:?} {:?}\n255", IMG_WIDTH, IMG_HEIGHT);
    write_render(&rendered_scene, SAMPLES_PER_PIXEL);
}
