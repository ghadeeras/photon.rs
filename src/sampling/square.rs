use rand::prelude::Distribution;
use rand::Rng;

use crate::basic::vectors::Vec3D;
use crate::sampling::PDF;

pub struct UniformSolidUnitSquare;

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
        let x = value.x();
        let y = value.y();
        let z = value.z();
        x >= 0.0 && x < 1.0 && y >= 0.0 && y < 1.0 && z == 0.0
    }

}
