use std::sync::Arc;

use crate::{Color, Ray, Vec3D};
use crate::materials::{Effect, Material};
use crate::things::Thing;

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
                    match hit.texture.material(&hit.hit, hit.geometry, hit.other_side_texture).effect_of(&hit.hit) {
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
                    r.origin = Vec3D::zero();
                    color *= self.environment.trace(&r);
                    depth = 0;
                }
            }
        }
        color
    }

}