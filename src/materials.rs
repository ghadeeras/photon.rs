use std::rc::Rc;

use crate::{Color, Constant};
use crate::brdfs::{BRDF, Lambertian};
use crate::geometries::Hit;

pub trait Material {

    fn effect_of(&self, hit: &Hit) -> Effect;

}

pub trait WrappedMaterial<M: Material> {

    fn as_texture_ref(&self) -> Rc<Constant<M>> {
        Rc::new(self.as_texture())
    }

    fn as_texture(&self) -> Constant<M>;

}

impl<M: Material> WrappedMaterial<M> for Rc<M> {

    fn as_texture(&self) -> Constant<M> {
        Constant(self.clone())
    }

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
