use std::f64::consts::{FRAC_1_PI, PI};

use rand::prelude::Distribution;
use rand::Rng;

use crate::basic::vectors::{Dot, Vec3D};
use crate::sampling::{PDF, UniformSolidUnitSquare};

pub struct UniformSolidUnitCircle;

impl Distribution<Vec3D> for UniformSolidUnitCircle {

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3D {
        let square_sample = rng.sample(UniformSolidUnitSquare);
        let angle = 2.0 * PI * square_sample.x();
        let radius = square_sample.y().sqrt();
        Vec3D::new(angle.cos() * radius, angle.sin() * radius, 0.0)
    }

}

impl PDF<Vec3D> for UniformSolidUnitCircle {

    fn pdf(&self, _: &Vec3D) -> f64 {
        FRAC_1_PI
    }

    fn contains(&self, value: &Vec3D) -> bool {
        value.length() < 1.0
    }

}
