use std::sync::Arc;

pub use fractal::*;
pub use simple::*;

use crate::basic::vectors::Vec3D;

mod simple;
mod fractal;

pub trait Noise: Send + Sync {

    fn value_at(&self, point: &Vec3D) -> f64;

}

impl Noise for Arc<dyn Noise> {

    fn value_at(&self, point: &Vec3D) -> f64 {
        self.as_ref().value_at(point)
    }

}
