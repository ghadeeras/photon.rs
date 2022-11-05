use std::rc::Rc;

use crate::cameras::{Camera, Exposure, Lens, Sensor};
use crate::colors::Color;
use crate::geometries::{GeometryWrapper, Sphere};
use crate::materials::{Diffusive, WrappedMaterial};
use crate::rays::Ray;
use crate::textures::Constant;
use crate::things::{AtomicThing, Things};
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
        let mut b = (ray.direction.unit().y() + 1.0) * 0.5;
        b *= b;
        Color::new(b, b, b)
    }

}

fn main() {
    let camera = Camera::new(
        Lens::ideal(1.0),
        Sensor::new(960, 720, 1.0),
        Exposure(0.0),
        64
    );
    let world = PathTraced {
        sky: Rc::new(Sky),
        thing: Rc::new(Things(vec![
            Rc::new(Sphere)
                .with_transformation(Translation(Vec3D::new(0.0, 1.0, -4.0)))
                .with_texture(Rc::new(Diffusive(Color::new(0.8, 0.3, 0.2))).as_texture()),
            Rc::new(Sphere)
                .with_transformation(Translation(Vec3D::new(0.0, -1.0, -4.0)))
                .with_texture(Rc::new(Diffusive(Color::new(0.2, 0.4, 0.8))).as_texture()),
        ])),
        depth: 16
    };
    let time = std::time::SystemTime::now();
    let image = camera.shoot(&world);
    println!("{:?}", time.elapsed());
    image.save("_image.png");
}
