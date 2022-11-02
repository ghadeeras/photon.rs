use std::ops::Mul;

use crate::Vec3D;
use crate::vectors::Dot;

pub struct Matrix {
    columns: [Vec3D; 3]
}

impl Matrix {

    pub fn new(x: &Vec3D, y: &Vec3D, z: &Vec3D) -> Self {
        Self {
            columns: [*x, *y, *z]
        }
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

    pub fn x(&self) -> &Vec3D {
        &self.columns[0]
    }

    pub fn y(&self) -> &Vec3D {
        &self.columns[1]
    }

    pub fn z(&self) -> &Vec3D {
        &self.columns[2]
    }

}

impl Mul<&Vec3D> for &Matrix {

    type Output = Vec3D;

    fn mul(self, rhs: &Vec3D) -> Self::Output {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

}
