use crate::basic::rays::Ray;
use crate::basic::vectors::Vec3D;
use crate::geometries::{Geometry, Hit};
use crate::transforms::{Transformation, Transformed};

impl<G: Geometry, T: Transformation> Geometry for Transformed<G, T> {

    fn shoot(&self, ray: &Ray, min: f64, max: f64) -> Option<Hit> {
        self.subject
            .shoot(&self.transformation.to_local(ray), min, max)
            .map(| hit | self.transformation.to_global(&hit))
    }

    fn surface_coordinates(&self, point: &Vec3D) -> Vec3D {
        self.subject.surface_coordinates(point)
    }

}
