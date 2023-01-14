use std::rc::Rc;
use std::sync::Arc;

pub use sphere::*;
pub use transformed::*;

use crate::basic::rays::Ray;
use crate::basic::vectors::Vec3D;

mod sphere;
mod transformed;

pub trait Geometry: Send + Sync {

    fn shoot(&self, ray: &Ray, min: f64, max: f64) -> Option<Hit>;

    fn surface_coordinates(&self, point: &Vec3D) -> Vec3D;

}

impl<G: Geometry> Geometry for Arc<G> {

    fn shoot(&self, ray: &Ray, min: f64, max: f64) -> Option<Hit> {
        self.as_ref().shoot(ray, min, max)
    }

    fn surface_coordinates(&self, point: &Vec3D) -> Vec3D {
        self.as_ref().surface_coordinates(point)
    }

}

#[derive(Clone, Debug)]
pub struct Hit {

    pub incident_ray: Ray,
    pub normal: Vec3D,
    pub distance: f64,
    pub outside: bool,

    local_hit: Option<Rc<Hit>>

}

impl Hit {

    pub fn new(outside: bool, normal: Vec3D, incident_ray: Ray, distance: f64) -> Self {
        Self { incident_ray, normal, distance, outside, local_hit: None }
    }

    pub fn transformed_as(&self, incident_ray: Ray, normal: Vec3D) -> Self {
        Self { incident_ray, normal, distance: self.distance, outside: self.outside, local_hit: Some(self.local_hit()) }
    }

    pub fn local_hit(&self) -> Rc<Self> {
        match self.local_hit {
            Some(ref hit) => hit.clone(),
            _ => Rc::new(self.clone())
        }
    }

}
