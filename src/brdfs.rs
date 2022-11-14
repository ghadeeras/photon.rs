use std::f64::consts::PI;

use rand::{Rng, thread_rng};

use crate::matrices::Matrix;
use crate::sampling::UniformSolidUnitSquare;
use crate::Vec3D;
use crate::vectors::Dot;

pub trait BRDF {

    fn sample(&self) -> (Vec3D, f64);

    fn pdf(&self, direction: &Vec3D) -> f64;

    fn direction_sample(&self) -> Vec3D {
        let (direction, _) = self.sample();
        direction
    }

}

pub struct Lambertian(pub Matrix);

impl Lambertian {

    pub fn new(surface_normal: &Vec3D) -> Lambertian {
        Lambertian(Matrix::with_z_alignment(surface_normal))
    }

}

impl BRDF for Lambertian {

    fn sample(&self) -> (Vec3D, f64) {
        let v = thread_rng().sample(UniformSolidUnitSquare);
        let c2 = v.x();
        let c = c2.sqrt();
        let s = (1.0 - c2).sqrt();
        let phi = 2.0 * PI * v.y();
        let &Lambertian(ref matrix) = self;
        (matrix * &Vec3D::new(s * phi.cos(), s * phi.sin(), c), c)
    }

    fn pdf(&self, direction: &Vec3D) -> f64 {
        let &Lambertian(ref matrix) = self;
        matrix.z().dot(direction)
    }

}