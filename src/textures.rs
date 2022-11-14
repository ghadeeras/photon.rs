use std::sync::Arc;

use crate::geometries::{Geometry, Hit};
use crate::materials::{Absorptive, Effect, Material};

pub trait Texture: Send + Sync {

    fn material<'a>(&'a self, hit: &'a Hit, geometry: &'a dyn Geometry, other_side_texture: &'a dyn Texture) -> MaterialHolder;

}

impl<T: Texture> Texture for Arc<T> {

    fn material<'a>(&'a self, hit: &'a Hit, geometry: &'a dyn Geometry, other_side_texture: &'a dyn Texture) -> MaterialHolder {
        self.as_ref().material(hit, geometry, other_side_texture)
    }

}

pub enum MaterialHolder<'a> {
    Ref(&'a dyn Material),
    New(Box<dyn Material>)
}

impl<'a> Material for MaterialHolder<'a> {

    fn effect_of(&self, hit: &Hit) -> Effect {
        let material = match self {
            &MaterialHolder::Ref(m) => m,
            &MaterialHolder::New(ref m) => m.as_ref()
        };
        material.effect_of(hit)
    }

}

pub struct Constant<M: Material>(pub M);
pub struct Black;
pub struct Same;

impl<M: Material> Texture for Constant<M> {

    fn material<'a>(&'a self, _: &'a Hit, _: &'a dyn Geometry, _: &'a dyn Texture) -> MaterialHolder {
        let &Constant(ref material) = self;
        MaterialHolder::Ref(material)
    }

}

impl Texture for Black {

    fn material<'a>(&'a self, _: &'a Hit, _: &'a dyn Geometry, _: &'a dyn Texture) -> MaterialHolder {
        return MaterialHolder::Ref(&Absorptive)
    }

}

impl Texture for Same {

    fn material<'a>(&'a self, hit: &'a Hit, geometry: &'a dyn Geometry, other_side_texture: &'a dyn Texture) -> MaterialHolder {
        other_side_texture.material(hit, geometry, &Black)
    }

}