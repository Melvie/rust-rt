use crate::utils::clamp;
use rand::{thread_rng, Rng};
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

const COLOUR_UTILITY_FLOAT: f64 = 256.0;

pub type Point3D = Vec3<f64>;
pub type Colour = Vec3<f64>;

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (COLOUR_UTILITY_FLOAT * self.x) as i16,
            (COLOUR_UTILITY_FLOAT * self.y) as i16,
            (COLOUR_UTILITY_FLOAT * self.z) as i16
        )
    }
}

impl Colour {
    pub fn write_colour(&self, sample_per_pixel: i16) -> Colour {
        let scale = 1.0 / (sample_per_pixel as f64);

        let scaled_colour: Colour = *self * scale;

        Colour {
            x: clamp(scaled_colour.x().sqrt(), 0.0, 0.999),
            y: clamp(scaled_colour.y().sqrt(), 0.0, 0.999),
            z: clamp(scaled_colour.z().sqrt(), 0.0, 0.999),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
}

impl Vec3<f64> {
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn length(&self) -> f64 {
        self.length_sqrd().sqrt()
    }
    pub fn length_sqrd(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn unit(&self) -> Self {
        self.div(self.length())
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, rhs: &Vec3<f64>) -> Vec3<f64> {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn random_from_range(min: f64, max: f64) -> Self {
        let mut rng = thread_rng();

        Vec3 {
            x: rng.gen_range(min..=max),
            y: rng.gen_range(min..=max),
            z: rng.gen_range(min..=max),
        }
    }
    pub fn random() -> Self {
        let mut rng = thread_rng();

        Vec3 {
            x: rng.gen_range(0.0..=1.0),
            y: rng.gen_range(0.0..=1.0),
            z: rng.gen_range(0.0..=1.0),
        }
    }
}

impl AddAssign for Vec3<f64> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl SubAssign for Vec3<f64> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl MulAssign for Vec3<f64> {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl DivAssign for Vec3<f64> {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}

impl Neg for Vec3<f64> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Add<Vec3<f64>> for Vec3<f64> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vec3<f64> {
    type Output = Self;
    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl Sub<Vec3<f64>> for Vec3<f64> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<&Vec3<f64>> for Vec3<f64> {
    type Output = Self;
    fn sub(self, other: &Self) -> Self {
        self - *other
    }
}

impl Mul<f64> for Vec3<f64> {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;
    fn mul(self, rhs: Vec3<f64>) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vec3<f64>> for Vec3<f64> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Div<f64> for Vec3<f64> {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        self * (1.0 / rhs)
    }
}

impl Sum for Vec3<f64> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(
            Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            |acc, other| acc + other,
        )
    }
}

pub trait Transpose {
    fn transpose(self) -> Self;
}

impl Transpose for Vec<Vec<Vec3<f64>>> {
    fn transpose(self) -> Self {
        assert!(!self.is_empty());
        (0..self[0].len())
            .map(|i| {
                self.iter()
                    .map(|inner| inner[i].clone())
                    .collect::<Vec<Vec3<f64>>>()
            })
            .collect()
    }
}
