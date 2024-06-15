use std::sync::Arc;

pub use atomic::*;
pub use composite::*;

use crate::basic::rays::Ray;
use crate::geometries::{Geometry, Hit};
use crate::textures::Texture;

mod atomic;
mod composite;
mod transformed;

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
