use photon::basic::colors::Color;
use photon::basic::rays::Ray;
use photon::basic::vectors::{Dot, Vec3D};
use photon::builders::Building;
use photon::cameras::{Camera, Exposure, Lens, Sensor};
use photon::geometries::Sphere;
use photon::materials::{Diffusive, Reflective, RefractionIndex, Refractive};
use photon::textures::Constant;
use photon::things::Things;
use photon::transforms::{AffineTransformation, Linear, Translation};
use photon::worlds::World;

struct Sky;

impl World for Sky {

    fn trace(&self, ray: &Ray) -> Color {
        let b = (ray.direction.unit().dot(Vec3D::new(0.48, 0.64, 0.6)) + 3.0) / 4.0;
        Color::grey_shade(b * b)
    }

}

pub fn main() {
    let camera = Camera {
        lens: Lens::ideal(1.0),
        sensor: Sensor::new(960, 720, 1.0),
        exposure: Exposure(0.0),
        samples_per_pixel: 64
    };
    let world = Building(Things(vec![
        Building(Sphere)
            .transformed(Linear::scaling(1.5, 1.5, 1.5).then_displacement_of(1.5, 0.0, 0.0))
            .with_outer_texture(Constant(Diffusive(Color::new(0.8, 0.4, 0.2))))
            .boxed(),
        Building(Sphere)
            .transformed(Linear::scaling(0.75, 0.75, 0.75).then_displacement_of(0.0, -0.75, 1.5))
            .with_texture(Constant(Refractive(Color::WHITE, RefractionIndex::of(1.5))))
            .boxed(),
        Building(Sphere)
            .transformed(Linear::scaling(3.0, 3.0, 3.0).then_displacement_of(-2.5, 1.5, -3.0))
            .with_outer_texture(Constant(Reflective(Color::new(0.8, 0.8, 0.8))))
            .boxed(),
        Building(Sphere)
            .transformed(Linear::scaling(16.0, 2.0, 16.0).then_displacement_of(1.5, -3.5, 0.0))
            .with_outer_texture(Constant(Diffusive(Color::new(0.2, 0.4, 0.8))))
            .boxed(),
    ]))
        .transformed(Translation::new(0.0, 0.0, -4.0))
        .with_environment_and_depth(Sky, 16)
        .done();
    let time = std::time::SystemTime::now();
    let image = camera.shoot(&world, 16);
    println!("{:?}", time.elapsed());
    image.save("_image_1.png");
}
