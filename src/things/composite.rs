use crate::basic::rays::Ray;
use crate::things::{MaterialHit, Thing};

pub struct Things(pub Vec<Box<dyn Thing>>);

impl Thing for Things {

    fn shoot(&self, ray: &Ray, min: f64, max: f64) -> Option<MaterialHit> {
        let mut hit = None;
        let mut max_distance = max;
        let &Things(ref things) = self;
        for thing in things.iter() {
            hit = thing.shoot(ray, min, max_distance).or(hit);
            max_distance = match hit {
                Some(ref h) => h.hit.distance,
                None => max_distance
            }
        }
        hit
    }

}

