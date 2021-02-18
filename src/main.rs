use indicatif::{ParallelProgressIterator, ProgressBar};
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use rust_rt::camera::Camera;
use rust_rt::objects::{Object, SceneObjects, Sphere};
use rust_rt::scene::Scene;
use rust_rt::vec3::{Colour, Point3D, Transpose, Vec3};

fn write_render(rendered_scene: &Vec<Colour>, samples_per_pxl: i16) {
    for render in rendered_scene {
        println!("{:}", render.write_colour(samples_per_pxl));
    }
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

    let scene = Scene::new(world, Camera::new(ASPECT_RATIO, 2.0, 1.0));

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
