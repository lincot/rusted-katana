//! <https://www.codewars.com/kata/58ee4962dc4f81d6f400001c/train/rust>

use core::ops::Add;

#[derive(Copy, Clone, PartialEq)]
pub struct Vector {
    pub i: f64,
    pub j: f64,
    pub k: f64,
}

impl Vector {
    pub const fn new(i: f64, j: f64, k: f64) -> Self {
        Self { i, j, k }
    }

    pub fn get_magnitude(&self) -> f64 {
        self.i
            .mul_add(self.i, self.j.mul_add(self.j, self.k * self.k))
            .sqrt()
    }

    pub const fn get_i() -> Self {
        Self {
            i: 1.,
            j: 0.,
            k: 0.,
        }
    }

    pub const fn get_j() -> Self {
        Self {
            i: 0.,
            j: 1.,
            k: 0.,
        }
    }

    pub const fn get_k() -> Self {
        Self {
            i: 0.,
            j: 0.,
            k: 1.,
        }
    }

    pub fn multiply_by_scalar(&self, scalar: f64) -> Self {
        Self {
            i: self.i * scalar,
            j: self.j * scalar,
            k: self.k * scalar,
        }
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.i
            .mul_add(other.i, self.j.mul_add(other.j, self.k * other.k))
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            i: self.j.mul_add(other.k, -(self.k * other.j)),
            j: self.k.mul_add(other.i, -(self.i * other.k)),
            k: self.i.mul_add(other.j, -(self.j * other.i)),
        }
    }

    pub fn is_parallel_to(&self, other: Self) -> bool {
        let cross = self.cross(other);
        [cross.i, cross.j, cross.k].iter().all(|x| x.abs() < 1e-6)
            && !self.is_zero()
            && !other.is_zero()
    }

    pub fn is_perpendicular_to(&self, other: Self) -> bool {
        self.dot(other).abs() < 1e-6 && !self.is_zero() && !other.is_zero()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.get_magnitude();
        Self {
            i: self.i / magnitude,
            j: self.j / magnitude,
            k: self.k / magnitude,
        }
    }

    pub fn is_normalized(&self) -> bool {
        (self.get_magnitude() - 1.).abs() < 1e-6
    }

    fn is_zero(&self) -> bool {
        self == &Self {
            i: 0.,
            j: 0.,
            k: 0.,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k,
        }
    }
}
