use std::rc::Rc;

use crate::{Ray, Vec3D};
use crate::geometries::Hit;

pub trait Transformation {

    fn to_local(&self, ray: &Ray) -> Ray;

    fn to_global(&self, hit: &Hit) -> Hit;

}

pub struct Translation(pub Vec3D);
pub struct Composite(pub Vec<Rc<dyn Transformation>>);

impl Transformation for Translation {

    fn to_local(&self, ray: &Ray) -> Ray {
        let &Translation(ref shift) = self;
        Ray::new(&ray.origin - shift, ray.direction, ray.time)
    }

    fn to_global(&self, hit: &Hit) -> Hit {
        let &Translation(ref shift) = self;
        let ray = Ray::new(&hit.incident_ray.origin + shift, hit.incident_ray.direction, hit.incident_ray.time);
        hit.local_hit().transformed_as(ray, hit.normal)
    }

}

impl Transformation for Composite {

    fn to_local(&self, ray: &Ray) -> Ray {
        let &Composite(ref transformations) = self;
        let mut ray = ray.clone();
        for t in transformations {
            ray = t.to_local(&ray);
        }
        ray
    }

    fn to_global(&self, hit: &Hit) -> Hit {
        let &Composite(ref transformations) = self;
        let mut hit = hit.clone();
        for t in transformations.iter() {
            hit = t.to_global(&hit);
        }
        hit
    }

}
