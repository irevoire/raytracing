use std::{fmt, ops};

use rand::Rng;

use crate::Interval;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec<const N: usize>([f64; N]);

impl<const N: usize> Default for Vec<N> {
    fn default() -> Self {
        Vec::new()
    }
}

impl<const N: usize> Vec<N> {
    pub fn new() -> Vec<N> {
        Vec([0.0; N])
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let mut vec = [0.0; N];

        for el in &mut vec {
            *el = rng.gen();
        }

        Vec(vec)
    }

    pub fn random_within_interval(interval: Interval) -> Self {
        let mut rng = rand::thread_rng();
        let mut vec = [0.0; N];

        for el in &mut vec {
            *el = rng.gen_range(interval.min..=interval.max);
        }

        Vec(vec)
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_within_interval(Interval::from(-1, 1));
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_on_hemisphere(normal: Self) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        // In the same hemisphere as the normal
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn dot(self, other: Self) -> f64 {
        let mut ret = 0.;

        for n in 0..N {
            ret += self[n] * other[n];
        }

        ret
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        let mut ret = 0.;
        for i in self.0 {
            ret += i * i;
        }
        ret
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        self.0.iter().all(|value| value.abs() < s)
    }
}

impl Vec<3> {
    pub fn from(a: impl TryInto<f64>, b: impl TryInto<f64>, c: impl TryInto<f64>) -> Self {
        Vec([
            a.try_into().map_err(|_| "Could not parse a").unwrap(),
            b.try_into().map_err(|_| "Could not parse b").unwrap(),
            c.try_into().map_err(|_| "Could not parse c").unwrap(),
        ])
    }

    pub const fn x(self) -> f64 {
        self.0[0]
    }

    pub const fn y(self) -> f64 {
        self.0[1]
    }

    pub const fn z(self) -> f64 {
        self.0[2]
    }

    pub const fn r(self) -> f64 {
        self.0[0]
    }

    pub const fn g(self) -> f64 {
        self.0[1]
    }

    pub const fn b(self) -> f64 {
        self.0[2]
    }

    pub fn cross(self, other: Self) -> Self {
        Self([
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        ])
    }
}

impl<const N: usize> ops::Neg for Vec<N> {
    type Output = Vec<N>;

    fn neg(self) -> Self::Output {
        Self(self.0.map(|n| -n))
    }
}

impl<const N: usize> ops::Add for Vec<N> {
    type Output = Vec<N>;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<const N: usize> ops::AddAssign for Vec<N> {
    fn add_assign(&mut self, rhs: Self) {
        for n in 0..N {
            self[n] += rhs[n];
        }
    }
}

impl<const N: usize> ops::Sub for Vec<N> {
    type Output = Vec<N>;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<const N: usize> ops::SubAssign for Vec<N> {
    fn sub_assign(&mut self, rhs: Self) {
        for n in 0..N {
            self[n] -= rhs[n];
        }
    }
}

impl<const N: usize> ops::Mul for Vec<N> {
    type Output = Vec<N>;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<const N: usize> ops::MulAssign for Vec<N> {
    fn mul_assign(&mut self, rhs: Self) {
        for n in 0..N {
            self[n] *= rhs[n];
        }
    }
}

impl<const N: usize> ops::Mul<f64> for Vec<N> {
    type Output = Vec<N>;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<const N: usize> ops::MulAssign<f64> for Vec<N> {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 = self.0.map(|n| n * rhs);
    }
}

impl<const N: usize> ops::Mul<Vec<N>> for f64 {
    type Output = Vec<N>;

    fn mul(self, rhs: Vec<N>) -> Self::Output {
        rhs * self
    }
}

impl<const N: usize> ops::Div<f64> for Vec<N> {
    type Output = Vec<N>;

    fn div(mut self, rhs: f64) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<const N: usize> ops::Div<Vec<N>> for f64 {
    type Output = Vec<N>;

    fn div(self, rhs: Vec<N>) -> Self::Output {
        1. / self * rhs
    }
}

impl<const N: usize> ops::DivAssign<f64> for Vec<N> {
    fn div_assign(&mut self, rhs: f64) {
        self.0 = self.0.map(|n| n / rhs);
    }
}

impl<const N: usize> ops::Deref for Vec<N> {
    type Target = [f64; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> ops::DerefMut for Vec<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Vec<3> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}
