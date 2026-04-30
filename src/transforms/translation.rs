use crate::basic::matrices::Matrix;
use crate::basic::rays::Ray;
use crate::basic::vectors::Vec3D;
use crate::geometries::Hit;
use crate::transforms::{Affine, AffineTransformation, Linear, Transformation};

pub struct Translation(pub Vec3D);

impl Translation {

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Translation(Vec3D::new(x, y, z))
    }

}

impl Transformation for Translation {

    fn to_local(&self, ray: &Ray) -> Ray {
        let Translation(ref displacement) = self;
        ray.with_origin(&ray.origin - displacement)
    }

    fn to_global(&self, hit: &Hit) -> Hit {
        let Translation(ref displacement) = self;
        let ray = hit.incident_ray.with_origin(&hit.incident_ray.origin + displacement);
        hit.local_hit().transformed_as(ray, hit.normal)
    }

}

impl AffineTransformation for Translation {

    type ThenLinear = Affine;
    type ThenTranslation = Translation;

    fn then_linear(self, matrix: Matrix) -> Self::ThenLinear {
        let Translation(ref d) = self;
        let displacement = &matrix * d;
        Affine(Linear::new(matrix), Translation(displacement))
    }

    fn then_translation(self, displacement: Vec3D) -> Self::ThenTranslation {
        let Translation(ref d) = self;
        Translation(&displacement + d)
    }

}

