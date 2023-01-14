use crate::basic::colors::Color;
use crate::geometries::Hit;
use crate::materials::{Effect, Material};

pub struct Emissive(pub Color);

impl Material for Emissive {

    fn effect_of(&self, _: &Hit) -> Effect {
        let Self(ref color) = self;
        Effect::Emission(*color)
    }

}
