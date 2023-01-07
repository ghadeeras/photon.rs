use std::sync::Arc;

use crate::basic::matrices::Matrix;
use crate::basic::rays::Ray;
use crate::basic::vectors::{Dot, Vec3D};
use crate::geometries::Hit;

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

pub struct Transformed<S, T: Transformation> {
    pub subject: S,
    pub transformation: T
}

pub struct Linear(Matrix, Matrix, f64);
pub struct Translation(pub Vec3D);
pub struct Affine(Linear, Translation);

impl Linear {

    pub fn new(matrix: Matrix) -> Self {
        let anti_matrix = matrix.anti_matrix();
        let det = matrix.x().dot(anti_matrix.x());
        Linear(matrix.clone(), anti_matrix, 1.0 / det)
    }

    pub fn omni_scaling(f: f64) -> Self {
        Self::scaling(f, f, f)
    }

    pub fn scaling(x: f64, y:f64, z: f64) -> Self {
        Self::new(Matrix::diagonal(x, y, z))
    }

    pub fn rotation(axis: &Vec3D, angle: f64) -> Self {
        Self::new(Matrix::rotation(axis, angle))
    }

}

impl Transformation for Linear {

    fn to_local(&self, ray: &Ray) -> Ray {
        let &Linear(_, ref anti_matrix, factor) = self;
        let origin = &ray.origin * anti_matrix * factor;
        let direction =  &ray.direction * anti_matrix * factor;
        Ray::new(origin, direction, ray.time)
    }

    fn to_global(&self, hit: &Hit) -> Hit {
        let &Linear(ref matrix, ref anti_matrix, _) = self;
        let origin = matrix * &hit.incident_ray.origin;
        let direction = matrix * &hit.incident_ray.direction;
        let ray = Ray::new(origin, direction, hit.incident_ray.time);
        hit.local_hit().transformed_as(ray, anti_matrix * &hit.normal)
    }

}

impl AffineTransformation for Linear {

    type ThenLinear = Linear;
    type ThenTranslation = Affine;

    fn then_linear(self, matrix: Matrix) -> Self::ThenLinear {
        let Linear(ref m, _, _) = self;
        Linear::new(&matrix * m)
    }

    fn then_translation(self, displacement: Vec3D) -> Self::ThenTranslation {
        Affine(self, Translation(displacement))
    }

}

impl Translation {

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Translation(Vec3D::new(x, y, z))
    }

}

impl Transformation for Translation {

    fn to_local(&self, ray: &Ray) -> Ray {
        let &Translation(ref displacement) = self;
        Ray::new(&ray.origin - displacement, ray.direction, ray.time)
    }

    fn to_global(&self, hit: &Hit) -> Hit {
        let &Translation(ref displacement) = self;
        let ray = Ray::new(&hit.incident_ray.origin + displacement, hit.incident_ray.direction, hit.incident_ray.time);
        hit.local_hit().transformed_as(ray, hit.normal)
    }

}

impl AffineTransformation for Translation {

    type ThenLinear = Affine;
    type ThenTranslation = Translation;

    fn then_linear(self, matrix: Matrix) -> Self::ThenLinear {
        let Translation(ref d) = self;
        let displacement = &matrix * d;
        Affine(Linear::new(matrix), Translation(displacement))
    }

    fn then_translation(self, displacement: Vec3D) -> Self::ThenTranslation {
        let Translation(ref d) = self;
        Translation(&displacement + d)
    }

}

impl Transformation for Affine {

    fn to_local(&self, ray: &Ray) -> Ray {
        let &Affine(ref linear, ref translation) = self;
        linear.to_local(&translation.to_local(ray))
    }

    fn to_global(&self, hit: &Hit) -> Hit {
        let &Affine(ref linear, ref translation) = self;
        translation.to_global(&linear.to_global(hit))
    }

}

impl AffineTransformation for Affine {

    type ThenLinear = Affine;
    type ThenTranslation = Affine;

    fn then_linear(self, matrix: Matrix) -> Self::ThenLinear {
        let Affine(Linear(ref m, _, _), Translation(ref d)) = self;
        Affine(Linear::new(&matrix * m), Translation(&matrix * d))
    }

    fn then_translation(self, displacement: Vec3D) -> Self::ThenTranslation {
        let Affine(linear, Translation(ref d)) = self;
        Affine(linear, Translation(&displacement + d))
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
