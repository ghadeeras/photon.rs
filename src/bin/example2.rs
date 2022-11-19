use std::f64::consts::PI;

use photon::builders::From;
use photon::cameras::{Camera, Exposure, Lens, Sensor};
use photon::colors::Color;
use photon::geometries::{Geometry, Hit, Sphere};
use photon::materials::{Composite, Diffusive, Emissive, Material, Reflective, RefractionIndex, Refractive};
use photon::rays::Ray;
use photon::textures::{Constant, MaterialHolder, Texture};
use photon::things::Things;
use photon::transforms::{AffineTransformation, Linear, Translation};
use photon::vectors::{Dot, Vec3D};
use photon::worlds::World;

struct Sky;

static GALAXY_AXIS: Vec3D = Vec3D::new(1.0, 1.0, -1.0);
static GALAXY_THINNESS: f64 = 8.0;
static GALAXY_BRIGHTNESS: f64 = 1.0 / 16.0;

impl World for Sky {

    fn trace(&self, ray: &Ray) -> Color {
        let alignment_with_galaxy = (1.0 - ray.direction.unit().dot(GALAXY_AXIS.unit()).powf(2.0)).powf(GALAXY_THINNESS);
        return Color::grey(alignment_with_galaxy * GALAXY_BRIGHTNESS);
    }

}

struct CheckerBoard<W: Material, B: Material>(W, B);

impl<W: Material, B: Material> Texture for CheckerBoard<W, B> {

    fn material<'a>(&'a self, hit: &'a Hit, geometry: &'a dyn Geometry, _: &'a dyn Texture) -> MaterialHolder {
        let &Self(ref w, ref b) = self;
        let point = geometry.surface_coordinates(&hit.local_hit().incident_ray.origin);
        let x = (5.0 * point.x() + 0.5).floor() as i32;
        let y = (5.0 * point.y() + 0.0).floor() as i32;
        MaterialHolder::Ref(if (x + y) & 1 == 0 { b } else { w })
    }

}

pub fn main() {
    let distance = 100.0;
    let camera = Camera {
        lens: Lens::ideal(30.0),
        sensor: Sensor::new(960, 720, 1.0),
        exposure: Exposure(0.0),
        samples_per_pixel: 32
    };
    let world = From(Things(vec![
        From(Sphere)
            .transformed(Translation::new(1.0, 1.0, 1.0))
            .with_texture(Constant(Refractive(Color::white(), RefractionIndex::of(1.5))))
            .boxed(),
        From(Sphere)
            .transformed(Linear::scaling(2.0, 2.0, 2.0)
                .then_displacement_of(-1.0, -1.0, -1.0))
            .with_texture(Constant(Diffusive(Color::new(0.2, 0.4, 0.8))))
            .boxed(),
        From(Sphere)
            .transformed(Linear::scaling(2.0, 1.0, 2.0)
                .then_rotation(&Vec3D::new(1.0, 0.0, -1.0), -PI/6.0)
                .then_displacement_of(-2.0, 2.0, -1.0)
            )
            .with_texture(CheckerBoard(
                Diffusive(Color::new(0.8, 0.4, 0.2)),
                Reflective(Color::new(1.0, 1.0, 0.0))
            ))
            .boxed(),
        From(Sphere)
            .transformed(Linear::scaling(2.0, 3.0, 2.0)
                .then_rotation(&Vec3D::new(2.0, 0.0, 1.0), -PI/6.0)
                .then_displacement_of(3.0, 0.0, -8.0))
            .with_texture(Constant(Composite::new(vec![
                (Box::new(Reflective(Color::white())), 0.7),
                (Box::new(Diffusive(Color::white())), 0.3),
            ])))
            .boxed(),
        From(Sphere)
            .transformed(Linear::scaling(10.0, 10.0, 10.0)
                .then_displacement_of(20.0, 20.0, 20.0))
            .with_outer_texture(Constant(Emissive(Color::grey(16.0))))
            .boxed(),
    ]))
        .with_transformed_geometry(Translation::new(-0.35, -0.2, -distance))
        .with_environment_and_depth(Sky, 16)
        .done();
    let time = std::time::SystemTime::now();
    let image = camera.shoot(&world, 16);
    println!("{:?}", time.elapsed());
    image.save("_image_2.png");
}