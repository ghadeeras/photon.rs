use crate::cameras::{Camera, Exposure, Lens, Sensor};
use crate::colors::Color;
use crate::geometries::{Sphere, Transformed};
use crate::materials::{Diffusive, Reflective};
use crate::rays::Ray;
use crate::textures::{Black, Constant};
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
    let camera = Camera {
        lens: Lens::ideal(1.0),
        sensor: Sensor::new(960, 720, 1.0),
        exposure: Exposure(0.0),
        samples_per_pixel: 64
    };
    let world = PathTraced {
        environment: Sky,
        subject: Things(vec![
            Box::new(AtomicThing {
                geometry: Transformed {
                    geometry: Sphere,
                    transformation: Translation(Vec3D::new(0.0, 1.0, -4.0)),
                },
                outer_texture: Constant(Diffusive(Color::new(0.2, 0.4, 0.8))),
                inner_texture: Black,
            }),
            Box::new(AtomicThing {
                geometry: Transformed {
                    geometry: Sphere,
                    transformation: Translation(Vec3D::new(0.0, -1.0, -4.0)),
                },
                outer_texture: Constant(Reflective(Color::white())),
                inner_texture: Black,
            }),
        ]),
        depth: 16
    };
    let time = std::time::SystemTime::now();
    let image = camera.shoot(&world, 16);
    println!("{:?}", time.elapsed());
    image.save("_image.png");
}
