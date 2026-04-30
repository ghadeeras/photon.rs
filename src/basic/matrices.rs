use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

use crate::basic::vectors::{Dot, Vec3D};

#[derive(Clone, Debug)]
pub struct Matrix {
    columns: [Vec3D; 3]
}

impl Matrix {

    pub fn new(x: &Vec3D, y: &Vec3D, z: &Vec3D) -> Self {
        Self {
            columns: [*x, *y, *z]
        }
    }

    pub fn identity() -> Self {
        Self::diagonal(1.0, 1.0, 1.0)
    }

    pub fn diagonal(x: f64, y: f64, z: f64) -> Self {
        Self::new(
            &Vec3D::new(x, 0.0, 0.0),
            &Vec3D::new(0.0, y, 0.0),
            &Vec3D::new(0.0, 0.0, z),
        )
    }

    pub fn with_z_alignment(z: &Vec3D) -> Self {
        let x2 = z.x() * z.x();
        let y2 = z.y() * z.y();
        let z2 = z.z() * z.z();

        let x1_l2 = z2 + y2;
        let x2_l2 = y2 + x2;
        let z_l2 = z2 + x2_l2;

        let zz = z / z_l2.sqrt();
        let xx = if x1_l2 > x2_l2 {
            Vec3D::new(0.0, z.z(), -z.y()) / x1_l2.sqrt()
        } else {
            Vec3D::new(z.y(), -z.x(), 0.0) / x2_l2.sqrt()
        };
        let yy = zz.cross(&xx);

        Self { columns: [xx, yy, zz] }
    }

    pub fn with_z_and_x_alignment(z: &Vec3D, x: &Vec3D) -> Self {
        let zz = z.unit();
        let xx = x.reject(&zz, true).unit();
        let yy = zz.cross(&xx);
        Self { columns: [xx, yy, zz] }
    }

    pub fn rotation(axis: &Vec3D, angle: f64) -> Self {
        let unit_axis = axis.unit();
        let cos = angle.cos();
        let sin = angle.sin();
        let x = unit_axis.x();
        let y = unit_axis.y();
        let z = unit_axis.z();
        let one_minus_cos = 1.0 - cos;
        let x1 = x * one_minus_cos;
        let y1 = y * one_minus_cos;
        let z1 = z * one_minus_cos;
        let xx = x * x1;
        let yy = y * y1;
        let zz = z * z1;
        let xy = x * y1;
        let yz = y * z1;
        let zx = z * x1;
        Self::new(
            &Vec3D::new(xx + cos, xy + z * sin, zx - y * sin),
            &Vec3D::new(xy - z * sin, yy + cos, yz + x * sin),
            &Vec3D::new(zx + y * sin, yz - x * sin, zz + cos)
        )
    }

    pub fn x(&self) -> &Vec3D {
        &self[0]
    }

    pub fn y(&self) -> &Vec3D {
        &self[1]
    }

    pub fn z(&self) -> &Vec3D {
        &self[2]
    }

    pub fn anti_matrix(&self) -> Matrix {
        let x = self.x();
        let y = self.y();
        let z = self.z();
        Matrix::new(&y.cross(z), &z.cross(x), &x.cross(y))
    }

    pub fn det(&self) -> f64 {
        self.x().cross(self.y()).dot(*self.z())
    }

    pub fn inverse(&self) -> Self {
        &self.anti_matrix() / self.det()
    }

}

impl Index<usize> for Matrix {

    type Output = Vec3D;

    fn index(&self, index: usize) -> &Self::Output {
        &self.columns[index]
    }

}

impl IndexMut<usize> for Matrix {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.columns[index]
    }

}

impl Neg for &Matrix {

    type Output = Matrix;

    fn neg(self) -> Self::Output {
        let x = -self.x();
        let y = -self.y();
        let z = -self.z();
        Self::Output::new(&x, &y, &z)
    }

}

impl Mul<&Vec3D> for &Matrix {

    type Output = Vec3D;

    fn mul(self, rhs: &Vec3D) -> Self::Output {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

}

impl Mul<&Matrix> for &Matrix {

    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let x = self * rhs.x();
        let y = self * rhs.y();
        let z = self * rhs.z();
        Self::Output::new(&x, &y, &z)
    }

}

impl Mul<f64> for &Matrix {

    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        let x = self.x() * rhs;
        let y = self.y() * rhs;
        let z = self.z() * rhs;
        Self::Output::new(&x, &y, &z)
    }

}

impl Mul<&Matrix> for f64 {

    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        rhs * self
    }

}

impl Div<f64> for &Matrix {

    type Output = Matrix;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }

}

impl Div<&Matrix> for f64 {

    type Output = Matrix;

    fn div(self, rhs: &Matrix) -> Self::Output {
        self * &rhs.inverse()
    }

}

impl Div<&Matrix> for &Matrix {

    type Output = Matrix;

    fn div(self, rhs: &Matrix) -> Self::Output {
        self * &rhs.inverse()
    }

}

impl Add<&Matrix> for &Matrix {

    type Output = Matrix;

    fn add(self, rhs: &Matrix) -> Self::Output {
        let x = self.x() + rhs.x();
        let y = self.y() + rhs.y();
        let z = self.z() + rhs.z();
        Self::Output::new(&x, &y, &z)
    }

}

impl Sub<&Matrix> for &Matrix {

    type Output = Matrix;

    fn sub(self, rhs: &Matrix) -> Self::Output {
        let x = self.x() - rhs.x();
        let y = self.y() - rhs.y();
        let z = self.z() - rhs.z();
        Self::Output::new(&x, &y, &z)
    }

}