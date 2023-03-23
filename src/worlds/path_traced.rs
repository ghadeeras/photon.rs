use rand::{Rng, thread_rng};

use crate::basic::colors::Color;
use crate::basic::rays::Ray;
use crate::basic::vectors::Vec3D;
use crate::brdfs::BRDF;
use crate::materials::{Effect, Material};
use crate::sampling::{Space, UniformUnitSphere};
use crate::things::{MaterialHit, Thing};
use crate::worlds::World;

pub struct PathTraced<W: World, T: Thing, S: ImportantDirectionSampler> {

    pub environment: W,
    pub subject: T,
    pub depth: u8,
    pub directions_sampler: S

}

impl<W: World, T: Thing, S: ImportantDirectionSampler> World for PathTraced<W, T, S> {

    fn trace(&self, ray: &Ray) -> Color {
        self.do_trace(ray, self.depth)
    }

}

impl<W: World, T: Thing, S: ImportantDirectionSampler> PathTraced<W, T, S> {

    fn do_trace(&self, ray: &Ray, depth: u8) -> Color {
        if depth > 0 {
            match self.subject.shoot(ray, 0.0001, f64::INFINITY) {
                Some(ref hit) => self.color_of(hit, depth),
                None => self.environment.trace(&ray.with_origin(Vec3D::zero())),
            }
        } else {
            Color::BLACK
        }
    }

    fn color_of(&self, hit: &MaterialHit, depth: u8) -> Color {
        let material_holder = hit.texture.material(&hit.hit, hit.geometry, hit.other_side_texture);
        match material_holder.effect_of(&hit.hit) {
            Effect::Absorption => Color::BLACK,
            Effect::Emission(c) => c,
            Effect::Scattering(c, ref brdf) => c * self.scatter(hit, brdf.as_ref(), depth),
            Effect::Redirection(c, direction) => c * self.redirect(hit, &direction, depth),
        }
    }

    fn scatter(&self, hit: &MaterialHit, brdf: &dyn BRDF, depth: u8) -> Color {
        let position = &hit.hit.incident_ray.origin;
        let (direction, weight) = self.directions_sampler.sample_direction_from(position, brdf);
        if weight == 0.0 {
            return Color::BLACK
        }
        let color = self.do_trace(&hit.hit.incident_ray.with_direction(direction), depth - 1);
        self.directions_sampler.feedback(position, &direction, &color);
        weight * color
    }

    fn redirect(&self, hit: &MaterialHit, direction: &Vec3D, depth: u8) -> Color {
        self.do_trace(&hit.hit.incident_ray.with_direction(*direction), depth - 1)
    }

}

pub trait ImportantDirectionSampler: Send + Sync {

    fn sample_direction_from(&self, position: &Vec3D, brdf: &dyn BRDF) -> (Vec3D, f64) {
        let directions = self.important_directions_at(position);
        let narrowness = brdf.narrowness();
        let dice: f64 = thread_rng().gen();
        let (direction, dir_pdf, brdf_pdf) = if dice < narrowness {
            let (direction, brdf_pdf) = brdf.arbitrary_sample_and_pdf();
            let dir_pdf = directions.pdf(&direction);
            (direction, dir_pdf, brdf_pdf)
        } else {
            let (direction, dir_pdf) = directions.arbitrary_sample_and_pdf();
            let brdf_pdf = brdf.pdf(&direction);
            (direction, dir_pdf, brdf_pdf)
        };
        let pdf = narrowness * brdf_pdf + (1.0 - narrowness) * dir_pdf;
        (direction, brdf_pdf / pdf)
    }

    fn important_directions_at(&self, position: &Vec3D) -> Box<dyn Space<Vec3D>>;

    fn feedback(&self, position: &Vec3D, direction: &Vec3D, color: &Color);

}

pub struct Omnidirectional;

impl ImportantDirectionSampler for Omnidirectional {

    fn sample_direction_from(&self, _: &Vec3D, brdf: &dyn BRDF) -> (Vec3D, f64) {
        (brdf.arbitrary_sample(), 1.0)
    }

    fn important_directions_at(&self, _: &Vec3D) -> Box<dyn Space<Vec3D>> {
        Box::new(UniformUnitSphere)
    }

    fn feedback(&self, _: &Vec3D, _: &Vec3D, _: &Color) {
    }

}
