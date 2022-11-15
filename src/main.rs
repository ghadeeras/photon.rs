use crate::cameras::{Camera, Exposure, Lens, Sensor};
use crate::colors::Color;
use crate::geometries::Sphere;
use crate::materials::{Diffusive, Reflective, RefractionIndex, Refractive};
use crate::rays::Ray;
use crate::textures::{Black, Constant, Same};
use crate::things::{AtomicThing, Things};
use crate::transforms::{Linear, Transformed, Translation};
use crate::vectors::{Dot, Vec3D};
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
        let b = (ray.direction.unit().dot(Vec3D::new(0.48, 0.64, 0.6)) + 3.0) / 4.0;
        Color::grey(b * b)
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
        subject: Transformed {
            transformation: Translation::new(0.0, 0.0, -4.0),
            subject: Things(vec![
                Box::new(AtomicThing {
                    geometry: Transformed {
                        subject: Sphere,
                        transformation: Linear::scaling(1.5, 1.5, 1.5)
                            .then(Translation::new(1.5, 0.0, 0.0)),
                    },
                    outer_texture: Constant(Diffusive(Color::new(0.8, 0.4, 0.2))),
                    inner_texture: Black,
                }),
                Box::new(AtomicThing {
                    geometry: Transformed {
                        subject: Sphere,
                        transformation: Linear::scaling(0.75, 0.75, 0.75)
                            .then(Translation::new(0.0, -0.75, 1.5)),
                    },
                    outer_texture: Constant(Refractive(Color::white(), RefractionIndex::of(1.5))),
                    inner_texture: Same,
                }),
                Box::new(AtomicThing {
                    geometry: Transformed {
                        subject: Sphere,
                        transformation: Linear::scaling(3.0, 3.0, 3.0)
                            .then(Translation::new(-2.5, 1.5, -3.0)),
                    },
                    outer_texture: Constant(Reflective(Color::new(0.8, 0.8, 0.8))),
                    inner_texture: Black,
                }),
                Box::new(AtomicThing {
                    geometry: Transformed {
                        subject: Sphere,
                        transformation: Linear::scaling(16.0, 2.0, 16.0)
                            .then(Translation::new(1.5, -3.5, 0.0)),
                    },
                    outer_texture: Constant(Diffusive(Color::new(0.2, 0.4, 0.8))),
                    inner_texture: Black,
                }),
            ]),
        },
        depth: 16
    };
    let time = std::time::SystemTime::now();
    let image = camera.shoot(&world, 16);
    println!("{:?}", time.elapsed());
    image.save("_image.png");
}
