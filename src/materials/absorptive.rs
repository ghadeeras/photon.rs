use crate::geometries::Hit;
use crate::materials::Effect::Absorption;
use crate::materials::{Effect, Material};

pub struct Absorptive;

impl Material for Absorptive {

    fn effect_of(&self, _: &Hit) -> Effect {
        Absorption
    }

}
