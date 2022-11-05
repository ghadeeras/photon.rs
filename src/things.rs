use std::rc::Rc;

use crate::geometries::{Geometry, Hit};
use crate::Ray;
use crate::textures::Texture;

pub trait Thing {

    fn shoot(&self, ray: &Ray, min: f64, max: f64) -> Option<MaterialHit>;

}

pub struct MaterialHit<'a> {
    pub hit: Hit,
    pub geometry: &'a dyn Geometry,
    pub texture: &'a dyn Texture
}

pub struct AtomicThing<G: Geometry, T: Texture> {

    pub geometry: Rc<G>,
    pub texture: Rc<T>

}

pub struct Things(pub Vec<Rc<dyn Thing>>);

impl<G: Geometry, T: Texture> Thing for AtomicThing<G, T> {

    fn shoot(&self, ray: &Ray, min: f64, max: f64) -> Option<MaterialHit> {
        match self.geometry.shoot(ray, min, max) {
            Some(ref hit) => Some(MaterialHit {
                hit: hit.clone(),
                geometry: self.geometry.as_ref(),
                texture: self.texture.as_ref(),
            }),
            None => None
        }

    }

}

impl Thing for Things {

    fn shoot(&self, ray: &Ray, min: f64, max: f64) -> Option<MaterialHit> {
        let mut hit = None;
        let mut max_distance = max;
        let &Things(ref things) = self;
        for thing in things.iter() {
            hit = thing.shoot(ray, min, max_distance).or(hit);
            max_distance = match hit {
                Some(ref h) => h.hit.distance,
                None => max_distance
            }
        }
        hit
    }

}
