use std::sync::Arc;

use crate::colors::Color;
use crate::materials::{Effect, Material};
use crate::rays::Ray;
use crate::things::Thing;
use crate::vectors::Vec3D;

pub trait World: Send + Sync {

    fn trace(&self, ray: &Ray) -> Color;

}

impl<W: World> World for Arc<W> {

    fn trace(&self, ray: &Ray) -> Color {
        self.as_ref().trace(ray)
    }

}

pub struct PitchBlack;

impl World for PitchBlack {

    fn trace(&self, _: &Ray) -> Color {
        Color::black()
    }

}

pub struct PathTraced<W: World, T: Thing> {

    pub environment: W,
    pub subject: T,
    pub depth: u8,

}

impl<W: World, T: Thing> World for PathTraced<W, T> {

    fn trace(&self, ray: &Ray) -> Color {
        let mut r = ray.clone();
        let mut depth = self.depth;
        let mut color = Color::white();
        while depth > 0 {
            match self.subject.shoot(&r, 0.0001, f64::INFINITY) {
                Some(ref hit) => {
                    let material_holder = hit.texture.material(&hit.hit, hit.geometry, hit.other_side_texture);
                    match material_holder.effect_of(&hit.hit) {
                        Effect::Absorption => {
                            color = Color::black();
                            break;
                        }
                        Effect::Emission(ref c) => {
                            color *= c;
                            break;
                        }
                        Effect::Scattering { color: ref c, ref brdf } => {
                            r.direction = brdf.sample_direction();
                            color *= c;
                        }
                        Effect::Redirection(ref c, ref d) => {
                            r.direction = *d;
                            color *= c;
                        }
                    }
                    r.origin = hit.hit.incident_ray.origin;
                },
                None => {
                    r.origin = Vec3D::zero();
                    color *= self.environment.trace(&r);
                    break;
                }
            }
            depth -= 1;
        }
        color
    }

}