use crate::basic::vectors::Vec3D;
use crate::noise::Noise;

pub struct Simple;

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
