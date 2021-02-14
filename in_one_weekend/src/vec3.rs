#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    pub fn random() -> Self {
        use crate::rand::random_double;

        Self(random_double(), random_double(), random_double())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        use crate::rand::random_double_range;

        Self(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1., 1.);
            if (p.length_squared() >= 1.) {
                continue;
            }
            return p;
        }
    }
}

use core::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, n: usize) -> &Self::Output {
        match n {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index failed: tried to access the {} position of a Vec3", n),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, n: usize) -> &mut Self::Output {
        match n {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index failed: tried to access the {} position of a Vec3", n),
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.0 /= other.0;
        self.1 /= other.1;
        self.2 /= other.2;
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

// Type aliases for vec3
pub type Point3 = Vec3; // 3D point
pub type Color = Vec3; // RGB color

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3(self * other.0, self * other.1, self * other.2)
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Self::Output {
        Vec3(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self(self.0 / other, self.1 / other, self.2 / other)
    }
}
