use std::ops::{Add, Div, Mul, Neg, Sub};

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

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3D { components: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.components[0]
    }

    pub fn y(&self) -> f64 {
        self.components[1]
    }

    pub fn z(&self) -> f64 {
        self.components[2]
    }

    pub fn unit(&self) -> Self {
        self / self.length()
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

