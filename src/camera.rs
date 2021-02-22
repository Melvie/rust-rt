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
    pub fn new(aspect_ratio: f64, vertical_fov: f64, origin: Point3D, target: Point3D, up: Vec3<f64>) -> Self {
        let viewport_height = 2.0 * (vertical_fov.to_radians() / 2.0).tan();
        let viewport = Viewport::new(aspect_ratio * viewport_height, viewport_height);

        let w = (origin - target).unit();
        let u = up.cross(&w).unit();
        let v = w.cross(&u);

        let focal_length = 1.0;

        let horizontal = viewport.width * u;
        let vertical = viewport.height * v;

        Camera {
            viewport,
            aspect_ratio,
            focal_length,
            origin,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - w,
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
