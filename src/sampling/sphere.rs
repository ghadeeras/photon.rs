use std::f64::consts::{FRAC_1_PI, PI};

use rand::{Rng, thread_rng};
use rand::prelude::Distribution;

use crate::basic::vectors::{Dot, Vec3D};
use crate::rough_equality;
use crate::sampling::{PDF, Space, UniformSolidUnitSquare};

pub struct UniformUnitSphere;

impl Distribution<Vec3D> for UniformUnitSphere {

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3D {
        Self::do_sample(rng)
    }

}

impl Space<Vec3D> for UniformUnitSphere {

    fn arbitrary_sample_and_pdf(&self) -> (Vec3D, f64) {
        let vector = self.arbitrary_sample();
        (vector, self.pdf(&vector))
    }

    fn arbitrary_sample(&self) -> Vec3D {
        Self::do_sample(&mut thread_rng())
    }

}

impl UniformUnitSphere {

    fn do_sample<R: Rng + ?Sized>(rng: &mut R) -> Vec3D {
        let unit_square_sample = rng.sample(UniformSolidUnitSquare);
        let cos_theta = 1.0 - 2.0 * unit_square_sample.x();
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let phi = 2.0 * PI * unit_square_sample.y();
        let (sin_phi, cos_phi) = phi.sin_cos();
        Vec3D::new(
            cos_theta * cos_phi,
            cos_theta * sin_phi,
            sin_theta
        )
    }

}

impl PDF<Vec3D> for UniformUnitSphere {

    fn pdf(&self, _: &Vec3D) -> f64 {
        FRAC_1_PI * 0.25
    }

    fn contains(&self, value: &Vec3D) -> bool {
        rough_equality(value.length(), 1.0)
    }

}
