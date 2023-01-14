use crate::basic::matrices::Matrix;
use crate::basic::vectors::Vec3D;
use crate::noise::Noise;

pub struct Fractal<N: Noise> {
    pub base: N,
    pub transformation: Matrix,
    pub displacement: Vec3D,
    pub fraction: f64,
    pub depth: u8,

    scalar: f64
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
