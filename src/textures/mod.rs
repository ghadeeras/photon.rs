use std::sync::Arc;

pub use black::*;
pub use constant::*;
pub use same::*;

use crate::geometries::{Geometry, Hit};
use crate::materials::MaterialHolder;

mod constant;
mod black;
mod same;

pub trait Texture: Send + Sync {

    fn material<'a>(&'a self, hit: &'a Hit, geometry: &'a dyn Geometry, other_side_texture: &'a dyn Texture) -> MaterialHolder;

}

impl<T: Texture> Texture for Arc<T> {

    fn material<'a>(&'a self, hit: &'a Hit, geometry: &'a dyn Geometry, other_side_texture: &'a dyn Texture) -> MaterialHolder {
        self.as_ref().material(hit, geometry, other_side_texture)
    }

}
