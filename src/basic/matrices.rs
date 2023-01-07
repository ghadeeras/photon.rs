use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

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
        let max_dot = 0.5 * z.length_squared();
        let x1 = Vec3D::new(z.z(), z.x(), z.y());
        let x2 = if z.dot(&x1) < max_dot { x1 } else { Vec3D::new(z.x(), -2.0 * z.y(), z.z()) };
        Self::with_z_and_x_alignment(&z, &x2)
    }

    pub fn with_z_and_x_alignment(z: &Vec3D, x: &Vec3D) -> Self {
        let zz = z.unit();
        let xx = x.reject(&zz, true).unit();
        let yy = zz.cross(&xx);
        Self::new(&xx, &yy, &zz)
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