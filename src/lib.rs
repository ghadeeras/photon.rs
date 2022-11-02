use std::rc::Rc;

use crate::cameras::{Camera, Exposure, Lens, Sensor};
use crate::colors::Color;
use crate::geometries::{Sphere, Transformed};
use crate::materials::Diffusive;
use crate::rays::Ray;
use crate::textures::Constant;
use crate::things::AtomicThing;
use crate::transforms::Translation;
use crate::vectors::Vec3D;
use crate::worlds::{PathTraced, World};

mod vectors;
mod matrices;
mod rays;
mod cameras;
mod sampling;
mod colors;
mod images;
mod worlds;
mod things;
mod geometries;
mod textures;
mod materials;
mod transforms;
mod brdfs;

struct Sky;

impl World for Sky {

    fn trace(&self, ray: &Ray) -> Color {
        let b = (ray.direction.unit().y() + 1.0) * 0.5;
        Color::new(b, b, b)
    }

}

#[test]
fn test() {
    let camera = Camera::new(
        Lens::ideal(1.0),
        Sensor::new(960, 720, 1.0),
        Exposure(0.0),
        64
    );
    let world = PathTraced {
        sky: Rc::new(Sky),
        thing: Rc::new(AtomicThing {
            geometry: Rc::new(Transformed {
                geometry: Rc::new(Sphere),
                transformation: Rc::new(Translation(Vec3D::new(0.0, 0.0, -4.0)))
            }),
            texture: Rc::new(Constant(Rc::new(Diffusive(Color::new(1.0, 0.0, 1.0)))))
        }),
        depth: 16
    };
    let time = std::time::SystemTime::now();
    let image = camera.shoot(&world);
    println!("{:?}", time.elapsed());
    image.save("_image.png");
}
