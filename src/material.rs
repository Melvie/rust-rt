use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::utils::random_unit_vec;
use crate::vec3::Colour;

pub trait Material {
    fn scatter(&self, hit_record: &HitRecord, ray: &Ray) -> Option<(Ray, Colour)>;
}

pub struct Metal {
    albedo: Colour,
}

pub struct Lambertian {
    albedo: Colour,
}

pub enum Materials {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material for Materials {
    fn scatter(&self, hit_record: &HitRecord, ray: &Ray) -> Option<(Ray, Colour)> {
        match self {
            Materials::Lambertian(lambertian) => lambertian.scatter(hit_record, ray),
            Materials::Metal(metal) => metal.scatter(hit_record, ray),
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
    pub fn new(albedo: Colour) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, hit_record: &HitRecord, ray: &Ray) -> Option<(Ray, Colour)> {
        let reflected_vec = ray.direction().unit().reflect(&hit_record.normal());
        let scattered_ray = Ray::new(hit_record.point(), reflected_vec);

        if scattered_ray.direction().dot(&hit_record.normal()) > 0.0 {
            Some((scattered_ray, self.albedo))
        } else {
            None
        }
    }
}
