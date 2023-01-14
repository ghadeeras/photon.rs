use rand::prelude::Distribution;
use rand::Rng;

use crate::basic::vectors::Vec3D;
use crate::sampling::UniformSolidUnitCircle;

pub struct Lens {
    pub aperture: f64,
    pub focal_length: f64,
    pub focal_plane_ratio: f64,
}

impl Lens {

    pub fn ideal(focal_length: f64) -> Self {
        Self::new(0.0, focal_length, focal_length)
    }

    pub fn new(aperture: f64, focal_length: f64, focal_plane_distance: f64) -> Self {
        Self {
            aperture,
            focal_length,
            focal_plane_ratio: focal_plane_distance / focal_length
        }
    }

}

impl Distribution<Vec3D> for Lens {

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3D {
        if self.aperture != 0.0 {
            self.aperture * rng.sample(UniformSolidUnitCircle)
        } else {
            Vec3D::zero()
        }
    }

}
