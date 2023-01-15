use crate::builders::Building;
use crate::geometries::Geometry;
use crate::textures::{Black, Same, Texture};
use crate::things::AtomicThing;

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
