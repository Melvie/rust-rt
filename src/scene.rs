use crate::camera::Camera;
use crate::objects::{Hit, SceneObjects};
use crate::vec3::Colour;
use crate::ray::Ray;
use rand::Rng;
use crate::utils::random_unit_sphere;



pub struct Scene {
	world: SceneObjects,
	camera: Camera<f64>
}

impl Scene {
	pub fn new(world: SceneObjects, camera: Camera<f64>) -> Scene {
		Scene {
			world,
			camera
		}
	}

	pub fn render(&self, max_depth: i16, img_width: i32, img_height: i32) -> Vec<Colour> {

        let pixels = (0..img_height)
        .rev()
        .flat_map(|j| (0..img_width).map(move |i| (i, j)));
        let mut rng = rand::thread_rng();

        pixels.map(|(i,j)| {
                let u = (i as f64 + rng.gen::<f64>()) / (img_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (img_height - 1) as f64;

                let ray = self.camera.get_ray(u, v);
                ray_colour(&ray, &self.world, max_depth)
        }).collect::<Vec<Colour>>()
	}
}

fn ray_colour(ray: &Ray, world: &SceneObjects, depth: i16) -> Colour {
    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(ray, 0.0, std::f64::INFINITY) {
        let target = hit_record.point() + hit_record.normal() + random_unit_sphere();

        return 0.5
            * ray_colour(
                &Ray::new(hit_record.point(), target - hit_record.point()),
                world,
                depth - 1,
            );
    }
    let unit_direction = ray.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
}