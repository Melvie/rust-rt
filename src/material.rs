use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::utils::{random_unit_sphere, random_unit_vec};
use crate::vec3::Colour;

pub trait Material {
    fn scatter(&self, hit_record: &HitRecord, ray: &Ray) -> Option<(Ray, Colour)>;
}

pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

pub struct Lambertian {
    albedo: Colour,
}

pub struct Dielectric {
    refraction_index: f64,
}

pub enum Materials {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material for Materials {
    fn scatter(&self, hit_record: &HitRecord, ray: &Ray) -> Option<(Ray, Colour)> {
        match self {
            Materials::Lambertian(lambertian) => lambertian.scatter(hit_record, ray),
            Materials::Metal(metal) => metal.scatter(hit_record, ray),
            Materials::Dielectric(dielectric) => dielectric.scatter(hit_record, ray),
        }
    }
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, hit_record: &HitRecord, _ray: &Ray) -> Option<(Ray, Colour)> {
        let mut scattered_dir = hit_record.normal() + random_unit_vec();

        if scattered_dir.near_zero() {
            scattered_dir = hit_record.normal()
        }

        Some((Ray::new(hit_record.point(), scattered_dir), self.albedo))
    }
}

impl Metal {
    pub fn new(albedo: Colour, fuzz_float: f64) -> Self {
        let fuzz = if fuzz_float < 1.0 { fuzz_float } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, hit_record: &HitRecord, ray: &Ray) -> Option<(Ray, Colour)> {
        let reflected_vec = ray.direction().unit().reflect(&hit_record.normal());
        let scattered_ray = Ray::new(
            hit_record.point(),
            reflected_vec + self.fuzz * random_unit_sphere(),
        );

        if scattered_ray.direction().dot(&hit_record.normal()) > 0.0 {
            Some((scattered_ray, self.albedo))
        } else {
            None
        }
    }
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, hit_record: &HitRecord, ray: &Ray) -> Option<(Ray, Colour)> {
        let refraction_ratio = if hit_record.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let refracted_ray = ray.interact(&hit_record, refraction_ratio);

        Some((refracted_ray, Colour::new(1.0, 1.0, 1.0)))
    }
}
