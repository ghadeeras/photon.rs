use std::rc::Rc;

use crate::geometries::{Geometry, Hit};
use crate::Ray;
use crate::textures::Texture;

pub trait Thing {

    fn shoot(&self, ray: &Ray, min: f64, max: f64) -> Option<MaterialHit>;

}

pub struct MaterialHit<'a> {
    pub hit: Hit,
    pub thing: &'a AtomicThing
}

pub struct AtomicThing {

    pub geometry: Rc<dyn Geometry>,
    pub texture: Rc<dyn Texture>

}

impl Thing for AtomicThing {

    fn shoot(&self, ray: &Ray, min: f64, max: f64) -> Option<MaterialHit> {
        match self.geometry.shoot(ray, min, max) {
            Some(ref hit) => Some(MaterialHit {
                hit: hit.clone(),
                thing: self
            }),
            None => None
        }

    }

}
