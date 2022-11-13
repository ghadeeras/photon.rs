use std::sync::Arc;

use crate::{Ray, Vec3D};
use crate::geometries::Hit;
use crate::matrices::Matrix;
use crate::vectors::Dot;

pub trait Transformation: Send + Sync {

    fn to_local(&self, ray: &Ray) -> Ray;

    fn to_global(&self, hit: &Hit) -> Hit;

}

impl<T: Transformation> Transformation for Arc<T> {

    fn to_local(&self, ray: &Ray) -> Ray {
        self.as_ref().to_local(ray)
    }

    fn to_global(&self, hit: &Hit) -> Hit {
        self.as_ref().to_global(hit)
    }

}

pub struct Translation(Vec3D);
pub struct Linear(Matrix, Matrix, f64);
pub struct Affine(Linear, Translation);
pub struct Composite(pub Vec<Box<dyn Transformation>>);

impl Translation {

    pub fn new(x: f64, y: f64, z: f64) -> Translation {
        Translation(Vec3D::new(x, y, z))
    }

}

impl Linear {

    pub fn of(matrix: &Matrix) -> Self {
        let anti_matrix = matrix.anti_matrix();
        let det = matrix.x().dot(anti_matrix.x());
        Linear(matrix.clone(), anti_matrix, 1.0 / det)
    }

    pub fn scaling(x: f64, y:f64, z: f64) -> Self {
        Linear::of(&Matrix::new(
            &Vec3D::new(x, 0.0, 0.0),
            &Vec3D::new(0.0, y, 0.0),
            &Vec3D::new(0.0, 0.0, z),
        ))
    }

    pub fn then(self, translation: Translation) -> Affine {
        Affine(self, translation)
    }

}

impl Transformation for Translation {

    fn to_local(&self, ray: &Ray) -> Ray {
        let &Translation(ref shift) = self;
        Ray::new(&ray.origin - shift, ray.direction, ray.time)
    }

    fn to_global(&self, hit: &Hit) -> Hit {
        let &Translation(ref shift) = self;
        let ray = Ray::new(&hit.incident_ray.origin + shift, hit.incident_ray.direction, hit.incident_ray.time);
        hit.local_hit().transformed_as(ray, hit.normal)
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

impl Transformation for Composite {

    fn to_local(&self, ray: &Ray) -> Ray {
        let &Composite(ref transformations) = self;
        let mut ray = ray.clone();
        for t in transformations {
            ray = t.to_local(&ray);
        }
        ray
    }

    fn to_global(&self, hit: &Hit) -> Hit {
        let &Composite(ref transformations) = self;
        let mut hit = hit.clone();
        for t in transformations.iter().rev() {
            hit = t.to_global(&hit);
        }
        hit
    }

}
