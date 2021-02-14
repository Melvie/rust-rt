use crate::vec3::{Point3D, Vec3};

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
}
