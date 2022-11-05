use std::rc::Rc;

use crate::geometries::{Geometry, Hit};
use crate::materials::Material;

pub trait Texture {

    fn material(&self, hit: &Hit, geometry: &dyn Geometry) -> &dyn Material;

}

pub struct Constant<M: Material>(pub Rc<M>);

impl<M: Material> Texture for Constant<M> {

    fn material(&self, _: &Hit, _: &dyn Geometry) -> &dyn Material {
        let &Constant(ref material) = self;
        material.as_ref()
    }

}

