use crate::basic::rays::Ray;
use crate::geometries::{Geometry, Hit};
use crate::textures::Texture;
use crate::things::{MaterialHit, Thing};

pub struct AtomicThing<G: Geometry, O: Texture, I: Texture> {
    pub geometry: G,
    pub outer_texture: O,
    pub inner_texture: I,
}

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

