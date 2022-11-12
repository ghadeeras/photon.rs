use std::sync::Arc;

use crate::geometries::{Geometry, Hit};
use crate::Ray;
use crate::textures::Texture;

pub trait Thing: Send + Sync {

    fn shoot(&self, ray: &Ray, min: f64, max: f64) -> Option<MaterialHit>;

}

impl<T: Thing> Thing for Arc<T> {

    fn shoot(&self, ray: &Ray, min: f64, max: f64) -> Option<MaterialHit> {
        self.as_ref().shoot(ray, min, max)
    }

}

pub struct MaterialHit<'a> {
    pub hit: Hit,
    pub geometry: &'a dyn Geometry,
    pub texture: &'a dyn Texture,
    pub other_side_texture: &'a dyn Texture,
}

pub struct AtomicThing<G: Geometry, O: Texture, I: Texture> {
    pub geometry: G,
    pub outer_texture: O,
    pub inner_texture: I,
}

pub struct Things(pub Vec<Box<dyn Thing>>);

impl<G: Geometry, O: Texture, I: Texture> Thing for AtomicThing<G, O, I> {

    fn shoot(&self, ray: &Ray, min: f64, max: f64) -> Option<MaterialHit> {
        match self.geometry.shoot(ray, min, max) {
            Some(hit) => if hit.outside {
                Some(MaterialHit {
                    hit,
                    geometry: &self.geometry,
                    texture: &self.outer_texture,
                    other_side_texture: &self.inner_texture,
                })
            } else {
                Some(MaterialHit {
                    hit,
                    geometry: &self.geometry,
                    texture: &self.inner_texture,
                    other_side_texture: &self.outer_texture,
                })
            },
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
