
use rust_rt::vec3::{Colour, Vec3, Point3D};
use rust_rt::ray::Ray;

pub fn ray_colour(ray: &Ray) -> Colour {
    let unit_direction = ray.direction().unit();
    let t = 0.5*(unit_direction.y() + 1.0);
    (1.0-t)*Colour::new(1.0, 1.0, 1.0) + t*Colour::new(0.5, 0.7, 1.0)
}

fn main() {

	const ASPECT_RATIO: f64 = 16.0 / 9.0;
        const IMG_WIDTH: i32 = 400;
        const IMG_HEIGHT: i32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as i32;
        const SAMPLES_PER_PIXEL: i16 = 100;
	const MAX_DEPTH: i16 = 50;

	let viewport_height = 2.0;
	let viewport_width = viewport_height * ASPECT_RATIO;
	let focal_length = 1.0;
	let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
	let vertical = Vec3::new(0.0, viewport_height, 0.0);
	let origin = Point3D::new(0.0, 0.0, 0.0);
	let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);


	println!("P3\n{:?} {:?}\n255", IMG_WIDTH, IMG_HEIGHT);

    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("Scanning lines remaining:{:?}", j);
        for i in 0..IMG_WIDTH {
			let u: f64 = i as f64 / (IMG_WIDTH - 1) as f64;
			let v: f64 = j as f64 / (IMG_HEIGHT - 1) as f64;

            let ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);

            println!("{:}", ray_colour(&ray));
        }
        eprint!("{}[2J", 27 as char);
    }
}