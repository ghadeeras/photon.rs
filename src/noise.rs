use std::sync::Arc;

use crate::basic::matrices::Matrix;
use crate::basic::vectors::Vec3D;

pub trait Noise: Send + Sync {

    fn value_at(&self, point: &Vec3D) -> f64;

}

pub struct Simple;

pub struct Fractal<N: Noise> {
    pub base: N,
    pub transformation: Matrix,
    pub displacement: Vec3D,
    pub fraction: f64,
    pub depth: u8,

    scalar: f64
}

impl Noise for Arc<dyn Noise> {

    fn value_at(&self, point: &Vec3D) -> f64 {
        self.as_ref().value_at(point)
    }

}

impl Noise for Simple {

    fn value_at(&self, point: &Vec3D) -> f64 {
        let v = Simple::vector_alias(point);
        v.x() * v.y() * v.z()
    }

}

impl Simple {

    fn vector_alias(v: &Vec3D) -> Vec3D {
        Vec3D::new(
            Self::component_alias(v.x()),
            Self::component_alias(v.y()),
            Self::component_alias(v.z()),
        )
    }

    fn component_alias(v: f64) -> f64 {
        let d = ((v - v.floor()) * 2.0 - 1.0).abs();
        d * d * (3.0 - 2.0 * d)
    }

}

impl<N: Noise> Noise for Fractal<N> {

    fn value_at(&self, point: &Vec3D) -> f64 {
        let mut p = *point;
        self.scalar * self.recursive_value_at(&mut p, self.depth)
    }

}

impl<N: Noise> Fractal<N> {

    pub fn new(base: N, transformation: Matrix, displacement: Vec3D, fraction: f64, depth: u8) -> Self {
        Self {
            base,
            transformation,
            displacement,
            fraction,
            depth,
            scalar: (1.0 - fraction) / (1.0 - fraction.powi(depth as i32 + 1))
        }
    }

    fn recursive_value_at(&self, point: &mut Vec3D, depth: u8) -> f64 {
        let result = self.base.value_at(point);
        if depth == 0 {
            result
        } else {
            *point = (&self.transformation * &*point) + self.displacement;
            result + self.fraction * self.recursive_value_at(point, depth - 1)
        }
    }

}