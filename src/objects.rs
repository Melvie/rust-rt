use crate::ray::Ray;
use crate::vec3::{Point3D, Vec3};

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct HitRecord {
    point: Point3D,
    normal: Vec3<f64>,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            point: Point3D::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_point(mut self, point: Vec3<f64>) -> HitRecord {
        self.point = point;
        self
    }

    pub fn set_time(mut self, time: f64) -> HitRecord {
        self.t = time;
        self
    }

    pub fn set_face_normal(mut self, ray: &Ray, outward_normal: &Vec3<f64>) -> HitRecord {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };

        self
    }

    pub fn normal(&self) -> Vec3<f64> {
        self.normal
    }

    pub fn point(&self) -> Point3D {
        self.point
    }
}

pub enum Object {
    Sphere(Sphere),
    SceneObjects(SceneObjects),
}

impl Hit for Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Object::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
            Object::SceneObjects(scene_object) => scene_object.hit(ray, t_min, t_max),
        }
    }
}

#[derive(Clone)]
pub struct Sphere {
    center: Point3D,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64) -> Self {
        Sphere { center, radius }
    }
    pub fn center(&self) -> Point3D {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3<f64> = ray.origin() - self.center;

        let a: f64 = ray.direction().length_sqrd();
        let half_b: f64 = oc.dot(&ray.direction());
        let c: f64 = oc.length_sqrd() - self.radius * self.radius;

        let discrim = half_b * half_b - a * c;
        if discrim < 0.0 {
            return None;
        }

        let discrim_sqrt = discrim.sqrt();

        let mut root = (-half_b - discrim_sqrt) / a;
        if root < t_min || t_max < root {
            root = (-half_b + discrim_sqrt) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let outward_normal: Vec3<f64> = (ray.at(root) - self.center) / self.radius;

        let hit_record = HitRecord::new()
            .set_time(root)
            .set_point(ray.at(root))
            .set_face_normal(ray, &outward_normal);

        Some(hit_record)
    }
}

pub struct SceneObjects {
    objects: Vec<Object>,
}

impl SceneObjects {
    pub fn new() -> Self {
        SceneObjects {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hit for SceneObjects {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record: Option<HitRecord> = None;

        for object in self.objects.iter() {
            if let Some(tmp_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = tmp_record.t;
                hit_record = Some(tmp_record)
            }
        }
        hit_record
    }
}
