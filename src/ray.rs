use crate::objects::HitRecord;
use crate::vec3::{Point3D, Vec3};
use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    origin: Point3D,
    direction: Vec3<f64>,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vec3<f64>) -> Self {
        Ray { origin, direction }
    }
    pub fn at(&self, t: f64) -> Point3D {
        return self.origin + self.direction * t;
    }

    pub fn direction(&self) -> Vec3<f64> {
        self.direction
    }

    pub fn origin(&self) -> Point3D {
        self.origin
    }

    pub fn interact(&self, hit_record: &HitRecord, refraction_ratio: f64) -> Ray {
        let unit_direction = self.direction().unit();
        let cos_theta = (-unit_direction).dot(&hit_record.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let direction =
            match refraction_ratio * sin_theta > 1.0 || reflectance(cos_theta, refraction_ratio) {
                true => unit_direction.reflect(&hit_record.normal()),
                false => unit_direction.refract(&hit_record.normal(), refraction_ratio),
            };

        Ray::new(hit_record.point(), direction)
    }
}

fn reflectance(cos: f64, refraction_ratio: f64) -> bool {
    let mut rng = rand::thread_rng();

    let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);

    r0 + (1.0 - r0) * ((1.0 - cos).powi(5)) > rng.gen::<f64>()
}
