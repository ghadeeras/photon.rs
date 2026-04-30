use crate::basic::colors::Color;
use crate::basic::vectors::Vec3D;

#[derive(Clone, Debug)]
pub struct Ray {
    pub origin: Vec3D,
    pub direction: Vec3D,
    pub color: Color,
    pub time: f64
}

impl Ray {

    pub fn new(origin: Vec3D, direction: Vec3D, color: Color, time: f64) -> Self {
        Self { origin, direction, color, time }
    }

    pub fn at(&self, distance: f64) -> Vec3D {
        self.origin + (distance * self.direction)
    }

    pub fn with_origin(&self, origin: Vec3D) -> Self {
        Self::new(origin, self.direction, self.color, self.time)
    }

    pub fn with_direction(&self, direction: Vec3D) -> Self {
        Self::new(self.origin, direction, self.color, self.time)
    }

    pub fn with_origin_and_direction(&self, origin: Vec3D, direction: Vec3D) -> Self {
        Self::new(origin, direction, self.color, self.time)
    }

    pub fn with_color(&self, color: Color) -> Self {
        Self::new(self.origin, self.direction, color, self.time)
    }

}