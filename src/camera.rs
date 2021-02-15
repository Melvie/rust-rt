use crate::ray::Ray;
use crate::vec3::{Point3D, Vec3};

pub struct Viewport<T> {
    height: T,
    width: T,
}

impl<T: Copy> Viewport<T> {
    pub fn new(width: T, height: T) -> Self {
        Viewport { width, height }
    }

    pub fn width(&self) -> T {
        self.width
    }

    pub fn height(&self) -> T {
        self.height
    }
}

#[allow(dead_code)]
pub struct Camera<T> {
    aspect_ratio: T,
    viewport: Viewport<T>,
    focal_length: T,
    origin: Point3D,
    lower_left_corner: Point3D,
    horizontal: Vec3<T>,
    vertical: Vec3<T>,
}

impl Camera<f64> {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Self {
        let viewport = Viewport::new(aspect_ratio * viewport_height, viewport_height);
        let horizontal = Vec3::new(viewport.width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let origin = Point3D::new(0.0, 0.0, 0.0);

        Camera {
            viewport,
            aspect_ratio,
            focal_length,
            origin,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vec3::new(0.0, 0.0, focal_length),
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
