use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::matrices::Matrix;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vec3D {
    pub components: [f64; 3]
}

pub trait Dot where Self: Copy {

    fn dot(self, rhs: Self) -> f64;

    fn length_squared(self) -> f64 {
        self.dot(self)
    }

    fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

}

impl Vec3D {

    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3D { components: [x, y, z] }
    }

    pub const fn x(&self) -> f64 {
        self.components[0]
    }

    pub const fn y(&self) -> f64 {
        self.components[1]
    }

    pub const fn z(&self) -> f64 {
        self.components[2]
    }

    pub fn unit(&self) -> Self {
        self / self.length()
    }

    pub fn cross(&self, rhs: &Vec3D) -> Vec3D {
        Vec3D::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }

    pub fn reject(&self, rhs: &Vec3D, rhs_normalized: bool) -> Vec3D {
        self - &self.project_on(rhs, rhs_normalized)
    }

    pub fn project_on(&self, rhs: &Vec3D, rhs_normalized: bool) -> Vec3D {
        let unit_rhs = if rhs_normalized {
            *rhs
        } else {
            rhs.unit()
        };
        unit_rhs * self.dot(&unit_rhs)
    }

}

impl Dot for &Vec3D {

    fn dot(self, rhs: Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

}

impl Dot for Vec3D {

    fn dot(self, rhs: Self) -> f64 {
        (&self).dot(&rhs)
    }

}

impl Add for &Vec3D {

    type Output = Vec3D;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z(),
        )
    }

}

impl Add for Vec3D {

    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }

}

impl Sub for &Vec3D {

    type Output = Vec3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z(),
        )
    }

}

impl Sub for Vec3D {

    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }

}

impl Neg for &Vec3D {

    type Output = Vec3D;

    fn neg(self) -> Self::Output {
        Self::Output::new(
            -self.x(),
            -self.y(),
            -self.z(),
        )
    }

}

impl Neg for Vec3D {

    type Output = Self;

    fn neg(self) -> Self::Output {
        (&self).neg()
    }

}

impl Mul<f64> for &Vec3D {

    type Output = Vec3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs,
        )
    }

}

impl Mul<f64> for Vec3D {

    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }

}

impl Mul<&Vec3D> for f64 {

    type Output = Vec3D;

    fn mul(self, rhs: &Vec3D) -> Self::Output {
        rhs * self
    }

}

impl Mul<Vec3D> for f64 {

    type Output = Vec3D;

    fn mul(self, rhs: Vec3D) -> Self::Output {
        &rhs * self
    }

}

impl Div<f64> for &Vec3D {

    type Output = Vec3D;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }

}

impl Div<f64> for Vec3D {

    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }

}

impl Mul<&Matrix> for &Vec3D {

    type Output = Vec3D;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        Self::Output::new(
            self.dot(rhs.x()),
            self.dot(rhs.y()),
            self.dot(rhs.z()),
        )
    }

}

#[cfg(test)]
pub mod tests {

    use proptest::{*, strategy::*};
    use super::*;

    prop_compose! {
        pub fn range()(v in -1.0..1.0) -> f64 {
            v
        }
    }

    prop_compose! {
        pub fn vec3()(x in range(), y in range(), z in range()) -> Vec3D {
            Vec3D::new(x, y, z)
        }
    }

    prop_compose! {
        pub fn non_zero_vec3()(v in vec3().prop_filter("non-zero vectors", |v| v.length() != 0.0)) -> Vec3D {
            v
        }
    }

    prop_compose! {
        pub fn unit_vec3()(v in non_zero_vec3()) -> Vec3D {
            v.unit()
        }
    }

}