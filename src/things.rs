use std::sync::Arc;

use crate::basic::rays::Ray;
use crate::geometries::{Geometry, Hit};
use crate::textures::Texture;
use crate::transforms::{Transformation, Transformed};

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
            Some(hit) => Some(if hit.outside {
                self.material_hit(hit, &self.outer_texture, &self.inner_texture)
            } else {
                self.material_hit(hit, &self.inner_texture, &self.outer_texture)
            }),
            None => None
        }

    }

}

impl<G: Geometry, O: Texture, I: Texture> AtomicThing<G, O, I> {

    fn material_hit<'a>(&'a self, hit: Hit, texture: &'a dyn Texture, other_side_texture: &'a dyn Texture) -> MaterialHit {
        MaterialHit {
            hit,
            geometry: &self.geometry,
            texture,
            other_side_texture
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

impl<T: Thing, F: Transformation> Thing for Transformed<T, F> {

    fn shoot(&self, ray: &Ray, min: f64, max: f64) -> Option<MaterialHit> {
        let local_ray = self.transformation.to_local(ray);
        let local_hit = self.subject.shoot(&local_ray, min, max);
        match local_hit {
            Some(ref h) => Some(MaterialHit {
                hit: self.transformation.to_global(&h.hit),
                geometry: h.geometry,
                texture: h.texture,
                other_side_texture: h.other_side_texture
            }),
            None => None
        }
    }

}
