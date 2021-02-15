use crate::vec3::{Point3D, Vec3};

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
    let mut point: Point3D = Vec3::random_from_range(-1.0, 1.0);

    while point.length_sqrd() >= 1.0 {
        point = Vec3::random_from_range(-1.0, 1.0);
    }
    return point;
}

pub fn random_unit_vec() -> Vec3<f64> {
    random_unit_sphere().unit()
}
