#![allow(dead_code)]

use std::sync::Arc;

use crate::geometries::Geometry;
use crate::textures::{Black, Same, Texture};
use crate::things::{AtomicThing, Thing};
use crate::transforms::{Transformation, Transformed};
use crate::worlds::{PathTraced, World};

pub struct Building<T>(pub T);

impl<T> Building<T> {

    pub fn done(self) -> T {
        let Building(value) = self;
        value
    }

    pub fn boxed(self) -> Box<T> {
        Box::new(self.done())
    }

    pub fn shared(self) -> Arc<T> {
        Arc::new(self.done())
    }

    pub fn transformed<F: Transformation>(self, transformation: F) -> Building<Transformed<T, F>> {
        Building(Transformed {
            subject: self.done(),
            transformation
        })
    }

}

impl<T: Thing> Building<T> {

    pub fn with_environment_and_depth<W: World>(self, environment: W, depth: u8) -> Building<PathTraced<W, T>> {
        Building(PathTraced {
            subject: self.done(),
            environment,
            depth
        })
    }

}

impl<G: Geometry> Building<G> {

    pub fn with_texture<T: Texture>(self, texture: T) -> Building<AtomicThing<G, T, Same>> {
        self.with_textures(texture, Same)
    }

    pub fn with_outer_texture<T: Texture>(self, texture: T) -> Building<AtomicThing<G, T, Black>> {
        self.with_textures(texture, Black)
    }

    pub fn with_inner_texture<T: Texture>(self, texture: T) -> Building<AtomicThing<G, Black, T>> {
        self.with_textures(Black, texture)
    }

    pub fn with_textures<O: Texture, I: Texture>(self, outer_texture: O, inner_texture: I) -> Building<AtomicThing<G, O, I>> {
        Building(AtomicThing {
            geometry: self.done(),
            outer_texture,
            inner_texture
        })
    }

}