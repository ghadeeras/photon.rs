#![allow(dead_code)]

use std::sync::Arc;
use crate::things::Thing;
use crate::{AtomicThing, Black, PathTraced, Same, Transformed, World};
use crate::geometries::Geometry;
use crate::textures::Texture;
use crate::transforms::Transformation;

pub struct From<T>(pub T);

impl<T> From<T> {

    pub fn done(self) -> T {
        let From(value) = self;
        value
    }

    pub fn boxed(self) -> Box<T> {
        Box::new(self.done())
    }

    pub fn shared(self) -> Arc<T> {
        Arc::new(self.done())
    }

}

impl<T: Thing> From<T> {

    pub fn with_environment_and_depth<W: World>(self, environment: W, depth: u8) -> From<PathTraced<W, T>> {
        From(PathTraced {
            subject: self.done(),
            environment,
            depth
        })
    }

    pub fn with_transformed_geometry<F: Transformation>(self, transformation: F) -> From<Transformed<T, F>> {
        From(Transformed {
            subject: self.done(),
            transformation
        })
    }

}

impl<G: Geometry> From<G> {

    pub fn transformed<T: Transformation>(self, transformation: T) -> From<Transformed<G, T>> {
        From(Transformed {
            subject: self.done(),
            transformation
        })
    }

    pub fn with_texture<T: Texture>(self, texture: T) -> From<AtomicThing<G, T, Same>> {
        self.with_textures(texture, Same)
    }

    pub fn with_outer_texture<T: Texture>(self, texture: T) -> From<AtomicThing<G, T, Black>> {
        self.with_textures(texture, Black)
    }

    pub fn with_inner_texture<T: Texture>(self, texture: T) -> From<AtomicThing<G, Black, T>> {
        self.with_textures(Black, texture)
    }

    pub fn with_textures<O: Texture, I: Texture>(self, outer_texture: O, inner_texture: I) -> From<AtomicThing<G, O, I>> {
        From(AtomicThing {
            geometry: self.done(),
            outer_texture,
            inner_texture
        })
    }

}