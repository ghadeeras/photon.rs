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

pub struct Things(pub Vec<Rc<dyn Thing>>);

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
