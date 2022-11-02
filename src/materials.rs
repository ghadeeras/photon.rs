use crate::Color;
use crate::brdfs::{BRDF, Lambertian};
use crate::geometries::Hit;

pub trait Material {

    fn effect_of(&self, hit: &Hit) -> Effect;

}

pub enum Effect {
    Emission(Color),
    Scattering {
        color: Color,
        brdf: Box<dyn BRDF>
    }
}

pub struct Emissive(pub Color);
pub struct Diffusive(pub Color);

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
