use crate::vec3::Vec3;
use rand::Rng;

pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

pub fn random_unit_sphere() -> Vec3<f64> {
    let mut point = Vec3::random_from_range(-1.0, 1.0);

    while point.length_sqrd() >= 1.0 {
        point = Vec3::random_from_range(-1.0, 1.0);
    }

    point
}

pub fn random_unit_vec() -> Vec3<f64> {
    random_unit_sphere().unit()
}

pub fn randon_unit_disk() -> Vec3<f64> {
    let mut rng = rand::thread_rng();

    let mut vec = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);

    while vec.length_sqrd() >= 1.0 {
        vec = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
    }

    vec
}
