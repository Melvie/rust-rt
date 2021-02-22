use crate::ray::Ray;
use crate::utils::randon_unit_disk;
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
    origin: Point3D,
    lower_left_corner: Point3D,
    horizontal: Vec3<T>,
    vertical: Vec3<T>,
    lens_radius: f64,
    u: Vec3<T>,
    v: Vec3<T>,
    w: Vec3<T>,
}

impl Camera<f64> {
    pub fn new(
        aspect_ratio: f64,
        vertical_fov: f64,
        origin: Point3D,
        target: Point3D,
        up: Vec3<f64>,
        apeture: f64,
        focus_dist: f64,
    ) -> Self {
        let viewport_height = 2.0 * (vertical_fov.to_radians() / 2.0).tan();
        let viewport = Viewport::new(aspect_ratio * viewport_height, viewport_height);

        let w = (origin - target).unit();
        let u = up.cross(&w).unit();
        let v = w.cross(&u);

        let horizontal = focus_dist * viewport.width * u;
        let vertical = focus_dist * viewport.height * v;

        Camera {
            viewport,
            aspect_ratio,
            origin,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist,
            horizontal,
            vertical,
            lens_radius: apeture / 2.0,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * randon_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
