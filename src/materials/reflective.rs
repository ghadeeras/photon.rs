use crate::basic::colors::Color;
use crate::geometries::Hit;
use crate::materials::{Effect, Material};

pub struct Reflective(pub Color);

impl Material for Reflective {

    fn effect_of(&self, hit: &Hit) -> Effect {
        let &Self(ref color) = self;
        let direction = hit.incident_ray.direction - 2.0 * hit.incident_ray.direction.project_on(&hit.normal, false);
        Effect::Redirection(*color, direction)
    }

}
