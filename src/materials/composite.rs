use rand::{rng, RngExt};

use crate::geometries::Hit;
use crate::materials::Effect::Absorption;
use crate::materials::{Effect, Material};

pub struct Composite(Vec<(Box<dyn Material>, f64)>);

impl Material for Composite {

    fn effect_of(&self, hit: &Hit) -> Effect {
        let Self(ref materials) = self;
        let choice: f64 = rng().random();
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
