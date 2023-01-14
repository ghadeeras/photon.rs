use crate::basic::rays::Ray;
use crate::things::{MaterialHit, Thing};
use crate::transforms::{Transformation, Transformed};

impl<T: Thing, F: Transformation> Thing for Transformed<T, F> {

    fn shoot(&self, ray: &Ray, min: f64, max: f64) -> Option<MaterialHit> {
        let local_ray = self.transformation.to_local(ray);
        let local_hit = self.subject.shoot(&local_ray, min, max);
        match local_hit {
            Some(ref h) => Some(MaterialHit {
                hit: self.transformation.to_global(&h.hit),
                geometry: h.geometry,
                texture: h.texture,
                other_side_texture: h.other_side_texture
            }),
            None => None
        }
    }

}
