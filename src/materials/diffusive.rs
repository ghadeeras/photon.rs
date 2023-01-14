use crate::basic::colors::Color;
use crate::brdfs::Lambertian;
use crate::geometries::Hit;
use crate::materials::{Effect, Material};

pub struct Diffusive(pub Color);

impl Material for Diffusive {

    fn effect_of(&self, hit: &Hit) -> Effect {
        let Self(ref color) = self;
        Effect::Scattering {
            color: *color,
            brdf: Box::new(Lambertian::new(&hit.normal))
        }
    }

}
