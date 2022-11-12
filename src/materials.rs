use std::sync::Arc;

use crate::brdfs::{BRDF, Lambertian};
use crate::Color;
use crate::geometries::Hit;

pub trait Material: Send + Sync {

    fn effect_of(&self, hit: &Hit) -> Effect;

}

impl<M: Material> Material for Arc<M> {

    fn effect_of(&self, hit: &Hit) -> Effect {
        self.as_ref().effect_of(hit)
    }

}

pub enum Effect {
    Emission(Color),
    Reflection(Color),
    Scattering {
        color: Color,
        brdf: Box<dyn BRDF>
    }
}

pub struct Emissive(pub Color);
pub struct Diffusive(pub Color);
pub struct Reflective(pub Color);

impl Material for Emissive {

    fn effect_of(&self, _: &Hit) -> Effect {
        let Self(ref color) = self;
        Effect::Emission(*color)
    }

}

impl Material for Diffusive {

    fn effect_of(&self, hit: &Hit) -> Effect {
        let Self(ref color) = self;
        Effect::Scattering {
            color: *color,
            brdf: Box::new(Lambertian::new(&hit.normal))
        }
    }

}

impl Material for Reflective {

    fn effect_of(&self, _: &Hit) -> Effect {
        let Self(ref color) = self;
        Effect::Reflection(*color)
    }

}