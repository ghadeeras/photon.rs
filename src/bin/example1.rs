use photon::builders::From;
use photon::cameras::{Camera, Exposure, Lens, Sensor};
use photon::colors::Color;
use photon::geometries::Sphere;
use photon::materials::{Diffusive, Reflective, RefractionIndex, Refractive};
use photon::rays::Ray;
use photon::textures::Constant;
use photon::things::Things;
use photon::transforms::{Linear, Translation};
use photon::vectors::{Dot, Vec3D};
use photon::worlds::World;

struct Sky;

impl World for Sky {

    fn trace(&self, ray: &Ray) -> Color {
        let b = (ray.direction.unit().dot(Vec3D::new(0.48, 0.64, 0.6)) + 3.0) / 4.0;
        Color::grey(b * b)
    }

}

pub fn main() {
    let camera = Camera {
        lens: Lens::ideal(1.0),
        sensor: Sensor::new(960, 720, 1.0),
        exposure: Exposure(0.0),
        samples_per_pixel: 64
    };
    let world = From(Things(vec![
        From(Sphere)
            .transformed(Linear::scaling(1.5, 1.5, 1.5).then(Translation::new(1.5, 0.0, 0.0)))
            .with_outer_texture(Constant(Diffusive(Color::new(0.8, 0.4, 0.2))))
            .boxed(),
        From(Sphere)
            .transformed(Linear::scaling(0.75, 0.75, 0.75).then(Translation::new(0.0, -0.75, 1.5)))
            .with_texture(Constant(Refractive(Color::white(), RefractionIndex::of(1.5))))
            .boxed(),
        From(Sphere)
            .transformed(Linear::scaling(3.0, 3.0, 3.0).then(Translation::new(-2.5, 1.5, -3.0)))
            .with_outer_texture(Constant(Reflective(Color::new(0.8, 0.8, 0.8))))
            .boxed(),
        From(Sphere)
            .transformed(Linear::scaling(16.0, 2.0, 16.0).then(Translation::new(1.5, -3.5, 0.0)))
            .with_outer_texture(Constant(Diffusive(Color::new(0.2, 0.4, 0.8))))
            .boxed(),
    ]))
        .with_transformed_geometry(Translation::new(0.0, 0.0, -4.0))
        .with_environment_and_depth(Sky, 16)
        .done();
    let time = std::time::SystemTime::now();
    let image = camera.shoot(&world, 16);
    println!("{:?}", time.elapsed());
    image.save("_image_1.png");
}
