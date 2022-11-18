use std::f64::consts::{FRAC_1_PI, PI};

use rand::prelude::Distribution;
use rand::Rng;

use crate::vectors::{Dot, Vec3D};

pub struct UniformSolidUnitSquare;
pub struct UniformSolidUnitCircle;

pub trait PDF<T> {

    fn pdf(&self, value: &T) -> f64;

    fn contains(&self, value: &T) -> bool;

    fn strict_pdf(&self, value: &T) -> f64 {
        if self.contains(value) {
            self.pdf(value)
        } else {
            0.0
        }
    }

}

impl Distribution<Vec3D> for UniformSolidUnitSquare {

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3D {
        let x = rng.gen();
        let y = rng.gen();
        Vec3D::new(x, y, 0.0)
    }

}

impl PDF<Vec3D> for UniformSolidUnitSquare {

    fn pdf(&self, _: &Vec3D) -> f64 {
        1.0
    }

    fn contains(&self, value: &Vec3D) -> bool {
        let &Vec3D { components: [x, y, z] } = value;
        x >= 0.0 && x < 1.0 && y >= 0.0 && y < 1.0 && z == 0.0
    }

}

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
