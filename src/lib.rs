use crate::cameras::{Camera, Exposure, Lens, Sensor};
use crate::colors::Color;
use crate::rays::Ray;
use crate::vectors::Vec3D;

mod vectors;
mod rays;
mod cameras;
mod sampling;
mod colors;
mod images;

pub trait World {

    fn trace(&self, ray: &Ray) -> Color;

}

struct Sky;

impl World for Sky {

    fn trace(&self, ray: &Ray) -> Color {
        let b = (ray.direction.y() + 3.0) * 0.25;
        Color::new(0.5, 0.5, b)
    }

}

#[test]
fn test() {
    let camera = Camera {
        lens: Lens::ideal(1.0),
        sensor: Sensor::new(960, 720, 1.0),
        exposure: Exposure(0.0),
        samples_per_pixel: 8
    };
    let image = camera.shoot(&Sky);
    image.save("_image.png")
}
