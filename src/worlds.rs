use std::rc::Rc;

use crate::{Color, Ray};
use crate::materials::Effect;
use crate::things::Thing;

pub trait World {

    fn trace(&self, ray: &Ray) -> Color;

}

pub struct PitchBlack;

impl World for PitchBlack {

    fn trace(&self, _: &Ray) -> Color {
        Color::black()
    }

}

pub struct PathTraced {

    pub sky: Rc<dyn World>,
    pub thing: Rc<dyn Thing>

}

impl World for PathTraced {

    fn trace(&self, ray: &Ray) -> Color {
        match self.thing.shoot(ray, 0.0001, f64::INFINITY) {
            Some(ref hit) => {
                match hit.thing.texture.material(hit).effect_of(&hit.hit) {
                    Effect::Emission(ref color) => *color
                }
            },
            None => self.sky.trace(ray)
        }
    }

}