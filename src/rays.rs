use crate::Vec3D;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3D,
    pub direction: Vec3D,
    pub time: f64
}

impl Ray {

    pub fn new(origin: Vec3D, direction: Vec3D, time: f64) -> Self {
        Ray { origin, direction, time }
    }

    pub fn at(&self, distance: f64) -> Vec3D {
        &self.origin + &(distance * &self.direction)
    }

}