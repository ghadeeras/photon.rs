use std::sync::Arc;

pub use absorptive::*;
pub use composite::*;
pub use diffusive::*;
pub use emissive::*;
pub use holder::*;
pub use reflective::*;
pub use refractive::*;

use crate::basic::colors::Color;
use crate::basic::vectors::Vec3D;
use crate::brdfs::BRDF;
use crate::geometries::Hit;

mod absorptive;
mod emissive;
mod diffusive;
mod reflective;
mod refractive;
mod composite;
mod holder;

pub trait Material: Send + Sync {

    fn effect_of(&self, hit: &Hit) -> Effect;

}

impl<M: Material> Material for Arc<M> {

    fn effect_of(&self, hit: &Hit) -> Effect {
        self.as_ref().effect_of(hit)
    }

}

pub enum Effect {
    Absorption,
    Emission(Color),
    Redirection(Color, Vec3D),
    Scattering {
        color: Color,
        brdf: Box<dyn BRDF>
    }
}
