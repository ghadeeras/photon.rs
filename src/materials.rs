use std::sync::Arc;

use rand::{Rng, thread_rng};

use crate::basic::colors::Color;
use crate::basic::vectors::{Dot, Vec3D};
use crate::brdfs::{BRDF, lambertian::Lambertian};
use crate::geometries::Hit;
use crate::materials::Effect::Absorption;

pub trait Material: Send + Sync {

    fn effect_of(&self, hit: &Hit) -> Effect;

}

impl<M: Material> Material for Arc<M> {

    fn effect_of(&self, hit: &Hit) -> Effect {
        self.as_ref().effect_of(hit)
    }

}

pub enum Effect {
    Absorption,
    Emission(Color),
    Redirection(Color, Vec3D),
    Scattering {
        color: Color,
        brdf: Box<dyn BRDF>
    }
}

pub struct Absorptive;
pub struct Emissive(pub Color);
pub struct Diffusive(pub Color);
pub struct Reflective(pub Color);
pub struct Refractive(pub Color, pub RefractionIndex);
pub struct Composite(Vec<(Box<dyn Material>, f64)>);

pub struct RefractionIndex(f64, f64, f64);

impl Material for Absorptive {

    fn effect_of(&self, _: &Hit) -> Effect {
        Absorption
    }

}

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

impl Material for Reflective {

    fn effect_of(&self, hit: &Hit) -> Effect {
        let &Self(ref color) = self;
        let direction = hit.incident_ray.direction - 2.0 * hit.incident_ray.direction.project_on(&hit.normal, false);
        Effect::Redirection(*color, direction)
    }

}

impl Material for Refractive {

    fn effect_of(&self, hit: &Hit) -> Effect {
        let &Self(ref color, ref index) = self;
        let direction = Self::redirection(&hit.incident_ray.direction, &hit.normal.unit(), index, hit.outside);
        Effect::Redirection(*color, direction)
    }

}

impl Refractive {

    fn redirection(incident: &Vec3D, normal: &Vec3D, index: &RefractionIndex, outside: bool) -> Vec3D {
        let &RefractionIndex(i, _, _) = index;
        let reciprocated_index = if outside { 1.0 / i } else { i };
        let incident_perpendicular_component = incident.project_on(normal, true);
        let incident_tangent_component = incident - &incident_perpendicular_component;
        let refraction_tangent_component = incident_tangent_component * reciprocated_index;
        let refraction_perpendicular_component_length_squared = Self::refraction_perpendicular_component_length_squared(&refraction_tangent_component, &incident_perpendicular_component, incident, index);
        if refraction_perpendicular_component_length_squared >= 0.0 {
            let refraction_perpendicular_component = normal * refraction_perpendicular_component_length_squared.sqrt();
            refraction_tangent_component - refraction_perpendicular_component
        } else {
            incident_tangent_component - incident_perpendicular_component
        }
    }

    fn refraction_perpendicular_component_length_squared(refraction_tangent_component: &Vec3D, incident_perpendicular_component: &Vec3D, incident_or_refraction: &Vec3D, index: &RefractionIndex) -> f64 {
        let incident_or_refraction_length_squared = incident_or_refraction.length_squared();
        let refraction_perpendicular_component_length_squared = incident_or_refraction_length_squared - refraction_tangent_component.length_squared();
        if refraction_perpendicular_component_length_squared >= 0.0 {
            let cos_angle = (incident_perpendicular_component.length_squared() / incident_or_refraction_length_squared).sqrt();
            if thread_rng().gen::<f64>() >= index.schlick_reflectance(cos_angle) {
                refraction_perpendicular_component_length_squared
            } else {
                -1.0
            }
        } else {
            refraction_perpendicular_component_length_squared
        }
    }

}

impl RefractionIndex {

    pub fn of(index: f64) -> Self {
        let i = (index - 1.0) / (index + 1.0);
        let i2 = i * i;
        Self(index, i2, 1.0 - i2)
    }

    fn schlick_reflectance(&self, cos_angle: f64) -> f64 {
        let &Self(_, c1, c2) = self;
        c1 + c2 * (1.0 - cos_angle).powf(5.0)
    }

}

impl Material for Composite {

    fn effect_of(&self, hit: &Hit) -> Effect {
        let &Self(ref materials) = self;
        let choice: f64 = thread_rng().gen();
        let mut sum = 0.0;
        for (material, weight) in materials {
            sum += weight;
            if sum >= choice {
                return material.effect_of(hit)
            }
        }
        Absorption
    }

}

impl Composite {

    pub fn new(materials: Vec<(Box<dyn Material>, f64)>) -> Self {
        let weights_sum: f64 = materials.iter().map(|(_, weight)| weight).sum();
        let new_materials: Vec<(Box<dyn Material>, f64)> = materials.into_iter()
            .map(|(m, weight)| (m, weight / weights_sum))
            .collect();
        Self(new_materials)
    }

}