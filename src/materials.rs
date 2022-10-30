use crate::Color;
use crate::geometries::Hit;
use crate::textures::Texture;
use crate::things::MaterialHit;

pub enum Effect {
    Emission(Color)
}

pub trait Material {

    fn effect_of(&self, hit: &Hit) -> Effect;

}

pub struct Emissive(pub Color);

impl Material for Emissive {

    fn effect_of(&self, _: &Hit) -> Effect {
        let Self(ref color) = self;
        Effect::Emission(*color)
    }

}

impl<M: Material> Texture for M {

    fn material(&self, _: &MaterialHit) -> &dyn Material {
        self
    }

}
