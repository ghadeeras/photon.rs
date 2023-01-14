use crate::basic::matrices::Matrix;
use crate::basic::rays::Ray;
use crate::basic::vectors::{Dot, Vec3D};
use crate::geometries::Hit;
use crate::transforms::{Affine, AffineTransformation, Transformation, Translation};

pub struct Linear(pub Matrix, pub Matrix, pub f64);

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
