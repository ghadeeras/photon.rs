use std::sync::Arc;

use crate::geometries::{Geometry, Hit};
use crate::materials::Material;

pub trait Texture: Send + Sync {

    fn material(&self, hit: &Hit, geometry: &dyn Geometry) -> &dyn Material;

}

impl<T: Texture> Texture for Arc<T> {

    fn material(&self, hit: &Hit, geometry: &dyn Geometry) -> &dyn Material {
        self.as_ref().material(hit, geometry)
    }

}

pub struct Constant<M: Material>(pub M);

impl<M: Material> Texture for Constant<M> {

    fn material(&self, _: &Hit, _: &dyn Geometry) -> &dyn Material {
        let &Constant(ref material) = self;
        material
    }

}

