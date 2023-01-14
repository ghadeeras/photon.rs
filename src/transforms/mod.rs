use std::sync::Arc;

pub use affine::*;
pub use linear::*;
pub use translation::*;

use crate::basic::matrices::Matrix;
use crate::basic::rays::Ray;
use crate::basic::vectors::Vec3D;
use crate::geometries::Hit;

mod linear;
mod translation;
mod affine;

pub trait Transformation: Send + Sync {

    fn to_local(&self, ray: &Ray) -> Ray;

    fn to_global(&self, hit: &Hit) -> Hit;

}

pub trait AffineTransformation: Transformation + Sized {

    type ThenLinear;
    type ThenTranslation;

    fn then_linear(self, matrix: Matrix) -> Self::ThenLinear;

    fn then_translation(self, displacement: Vec3D) -> Self::ThenTranslation;

    fn then_scaling(self, x: f64, y:f64, z: f64) -> Self::ThenLinear {
        self.then_linear(Matrix::diagonal(x, y, z))
    }

    fn then_omni_scaling(self, f: f64) -> Self::ThenLinear {
        self.then_scaling(f, f, f)
    }

    fn then_rotation(self, axis: &Vec3D, angle: f64) -> Self::ThenLinear {
        self.then_linear(Matrix::rotation(axis, angle))
    }

    fn then_displacement_of(self, x: f64, y: f64, z: f64) -> Self::ThenTranslation {
        self.then_translation(Vec3D::new(x, y, z))
    }

}

impl<T: Transformation> Transformation for Arc<T> {

    fn to_local(&self, ray: &Ray) -> Ray {
        self.as_ref().to_local(ray)
    }

    fn to_global(&self, hit: &Hit) -> Hit {
        self.as_ref().to_global(hit)
    }

}

pub struct Transformed<S, T: Transformation> {
    pub subject: S,
    pub transformation: T
}
