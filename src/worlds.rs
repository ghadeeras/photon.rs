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
    pub thing: Rc<dyn Thing>,
    pub depth: u8,

}

impl World for PathTraced {

    fn trace(&self, ray: &Ray) -> Color {
        let mut r = ray.clone();
        let mut depth = self.depth;
        let mut color = Color::white();
        while depth > 0 {
            match self.thing.shoot(&r, 0.0001, f64::INFINITY) {
                Some(ref hit) => {
                    match hit.thing.texture.material(hit).effect_of(&hit.hit) {
                        Effect::Emission(ref c) => {
                            color = *c;
                            depth = 0;
                        },
                        Effect::Scattering { color: ref c, ref brdf } => {
                            let (direction, _) = brdf.sample();
                            r.origin = hit.hit.incident_ray.origin;
                            r.direction = direction;
                            color *= c;
                            depth -= 1;
                        }
                    }
                },
                None => {
                    color *= self.sky.trace(&r);
                    depth = 0;
                }
            }
        }
        color
    }

}