use crate::geometries::Hit;
use crate::materials::{Effect, Material};
use crate::materials::Effect::Absorption;

pub struct Absorptive;

impl Material for Absorptive {

    fn effect_of(&self, _: &Hit) -> Effect {
        Absorption
    }

}
