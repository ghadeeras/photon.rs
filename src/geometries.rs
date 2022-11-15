use std::f64::consts::PI;
use std::rc::Rc;
use std::sync::Arc;

use crate::{Ray, Vec3D};
use crate::transforms::{Transformation, Transformed};
use crate::vectors::Dot;

pub trait Geometry: Send + Sync {

    fn shoot(&self, ray: &Ray, min: f64, max: f64) -> Option<Hit>;

    fn surface_coordinates(&self, point: &Vec3D) -> Vec3D {
        *point
    }

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

impl<G: Geometry, T: Transformation> Geometry for Transformed<G, T> {

    fn shoot(&self, ray: &Ray, min: f64, max: f64) -> Option<Hit> {
        self.subject
            .shoot(&self.transformation.to_local(ray), min, max)
            .map(| hit | self.transformation.to_global(&hit))
    }

}

pub struct Sphere;

impl Geometry for Sphere {

    fn shoot(&self, ray: &Ray, min: f64, max: f64) -> Option<Hit> {
        let direction_length_squared = ray.direction.length_squared();
        let half_b = ray.direction.dot(ray.origin) / direction_length_squared;
        let c = (ray.origin.length_squared() - 1.0) / direction_length_squared;
        if c != 0.0 {
            let d = half_b * half_b - c;
            if d > 0.0 {
                let sqrt_d = d.sqrt();
                let incident = Sphere::incident(true, ray, -half_b - sqrt_d, min, max);
                if let None = incident {
                    Sphere::incident(false, ray, -half_b + sqrt_d, min, max)
                } else {
                    incident
                }
            } else {
                None
            }
        } else {
            Sphere::incident(false, ray, -2.0 * half_b, min, max)
        }
    }

}

impl Sphere {

    fn incident(outside: bool, ray: &Ray, distance: f64, min: f64, max: f64) -> Option<Hit> {
        if min < distance && distance < max {
            Some(Self::hit(outside, ray, distance))
        } else {
            None
        }
    }

    fn hit(outside: bool, ray: &Ray, distance: f64) -> Hit {
        let point = ray.at(distance);
        let distance_to_center = ray.origin.length();
        let area = if outside {
            2.0 * PI * (1.0 - 1.0 / distance_to_center)
        } else {
            -4.0 * PI
        };
        Hit::new(outside, area * point, Ray::new(point, ray.direction, ray.time), distance)
    }

}
