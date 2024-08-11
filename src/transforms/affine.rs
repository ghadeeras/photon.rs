use crate::basic::matrices::Matrix;
use crate::basic::rays::Ray;
use crate::basic::vectors::Vec3D;
use crate::geometries::Hit;
use crate::transforms::{AffineTransformation, Linear, Transformation, Translation};

pub struct Affine(pub Linear, pub Translation);

impl Transformation for Affine {

    fn to_local(&self, ray: &Ray) -> Ray {
        let Affine(ref linear, ref translation) = self;
        linear.to_local(&translation.to_local(ray))
    }

    fn to_global(&self, hit: &Hit) -> Hit {
        let Affine(ref linear, ref translation) = self;
        translation.to_global(&linear.to_global(hit))
    }

}

impl AffineTransformation for Affine {

    type ThenLinear = Affine;
    type ThenTranslation = Affine;

    fn then_linear(self, matrix: Matrix) -> Self::ThenLinear {
        let Affine(Linear(ref m, _, _), Translation(ref d)) = self;
        Affine(Linear::new(&matrix * m), Translation(&matrix * d))
    }

    fn then_translation(self, displacement: Vec3D) -> Self::ThenTranslation {
        let Affine(linear, Translation(ref d)) = self;
        Affine(linear, Translation(&displacement + d))
    }

}

