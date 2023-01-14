use std::f64::consts::PI;

use crate::basic::rays::Ray;
use crate::basic::vectors::{Dot, Vec3D};
use crate::geometries::{Geometry, Hit};

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
                let hit = Sphere::possible_hit(true, ray, -half_b - sqrt_d, min, max);
                match hit {
                    None => Sphere::possible_hit(false, ray, -half_b + sqrt_d, min, max),
                    _ => hit
                }
            } else {
                None
            }
        } else {
            Sphere::possible_hit(false, ray, -2.0 * half_b, min, max)
        }
    }

    fn surface_coordinates(&self, point: &Vec3D) -> Vec3D {
        let x = point.x();
        let y = point.y();
        let z = point.z();
        let a = x.atan2(z) / PI;
        let b = y.atan2((x * x + z * z).sqrt()) / PI;
        Vec3D::new(a, b, 0.0)
    }

}

impl Sphere {

    fn possible_hit(outside: bool, ray: &Ray, distance: f64, min: f64, max: f64) -> Option<Hit> {
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
