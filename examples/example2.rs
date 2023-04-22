use std::f64::consts::{PI, SQRT_2};

use photon::basic::colors::Color;
use photon::basic::matrices::Matrix;
use photon::basic::rays::Ray;
use photon::basic::vectors::{Dot, Vec3D};
use photon::builders::Building;
use photon::geometries::{Geometry, Hit, Sphere};
use photon::materials::{Diffusive, Emissive, Material, MaterialHolder, Reflective, RefractionIndex, Refractive};
use photon::noise::{Fractal, Noise, Simple};
use photon::textures::{Constant, Texture};
use photon::things::Things;
use photon::transforms::{AffineTransformation, Linear, Translation};
use photon::viewing::{Camera, Exposure, Lens, Sensor};
use photon::worlds::World;

struct Sky;

static GALAXY_AXIS: Vec3D = Vec3D::new(1.0, 1.0, -1.0);
static GALAXY_THINNESS: f64 = 8.0;
static GALAXY_BRIGHTNESS: f64 = 1.0 / 16.0;

impl World for Sky {

    fn trace(&self, ray: &Ray) -> Color {
        let alignment_with_galaxy = (1.0 - ray.direction.unit().dot(GALAXY_AXIS.unit()).powf(2.0)).powf(GALAXY_THINNESS);
        return Color::grey_shade(alignment_with_galaxy * GALAXY_BRIGHTNESS);
    }

}

struct CheckerBoard<W: Material, B: Material>(W, B);

impl<W: Material, B: Material> Texture for CheckerBoard<W, B> {

    fn material<'a>(&'a self, hit: &'a Hit, geometry: &'a dyn Geometry, _: &'a dyn Texture) -> MaterialHolder {
        let &Self(ref w, ref b) = self;
        let point = geometry.surface_coordinates(&hit.local_hit().incident_ray.origin);
        let x = (5.0 * point.x() + 0.5).floor() as i32;
        let y = (5.0 * point.y() + 0.0).floor() as i32;
        MaterialHolder::Borrowing(if (x + y) & 1 == 0 { b } else { w })
    }

}

struct PlanetCrust {
    noise: Fractal<Simple>,
    land: Color,
    sea: Color,
    sea_level: f64,
    detail: f64
}

impl Texture for PlanetCrust {

    fn material<'a>(&'a self, hit: &'a Hit, _: &'a dyn Geometry, _: &'a dyn Texture) -> MaterialHolder {
        let noise = self.noise.value_at(&(hit.incident_ray.origin * self.detail)) * 2.0;
        let level = ((noise - noise.floor()) * 2.0 - 1.0).abs();
        let smooth_level = level * level * (3.0 - 2.0 * level);
        MaterialHolder::Owning(if smooth_level > self.sea_level {
            Box::new(Diffusive(smooth_level * self.land))
        } else {
            Box::new(Reflective(self.sea))
        })
    }

}

struct Woody {
    noise: Fractal<Simple>,
    color: Color,
    freq: f64,
    detail: f64
}

impl Texture for Woody {

    fn material<'a>(&'a self, hit: &'a Hit, _: &'a dyn Geometry, _: &'a dyn Texture) -> MaterialHolder {
        let noise = self.noise.value_at(&(hit.incident_ray.origin * self.detail)) * self.freq;
        MaterialHolder::Owning(Box::new(Diffusive(noise.fract() * self.color)))
    }

}

fn fractal_noise(depth: u8) -> Fractal<Simple> {
    let s = SQRT_2;
    Fractal::new(
        Simple,
        &Matrix::with_z_alignment(&Vec3D::new(3.0, 2.0, 1.0)) * &Matrix::diagonal(s, s, s),
        Vec3D::new(0.4, 0.5, 0.6),
        1.0 / s,
        depth
    )
}

pub fn main() {
    let distance = 100.0;
    let camera = Camera {
        lens: Lens::ideal(30.0),
        sensor: Sensor::new(960, 720, 1.0),
        exposure: Exposure(0.0),
        samples_per_pixel: 32
    };
    let world = Building(Things(vec![
        Building(Sphere)
            .transformed(Translation::new(1.0, 1.0, 1.0))
            .with_texture(Constant(Refractive(Color::WHITE, RefractionIndex::of(1.5))))
            .boxed(),
        Building(Sphere)
            .transformed(Linear::scaling(2.0, 2.0, 2.0)
                .then_displacement_of(-1.0, -1.0, -1.0))
            .with_texture(Woody {
                noise: fractal_noise(4),
                color: Color::new(0.2, 0.4, 0.8),
                freq: 32.0,
                detail: 0.5
            })
            .boxed(),
        Building(Sphere)
            .transformed(Linear::scaling(2.0, 1.0, 2.0)
                .then_rotation(&Vec3D::new(1.0, 0.0, -1.0), -PI/6.0)
                .then_displacement_of(-2.0, 2.0, -1.0)
            )
            .with_texture(CheckerBoard(
                Diffusive(Color::new(0.8, 0.4, 0.2)),
                Reflective(Color::new(1.0, 1.0, 0.1))
            ))
            .boxed(),
        Building(Sphere)
            .transformed(Linear::scaling(2.0, 3.0, 2.0)
                .then_rotation(&Vec3D::new(2.0, 0.0, 1.0), -PI/6.0)
                .then_displacement_of(3.0, 0.0, -8.0))
            .with_texture(PlanetCrust {
                noise: fractal_noise(16),
                land: Color::new(0.4, 0.8, 0.2),
                sea: Color::new(0.1, 1.0, 1.0),
                sea_level: 0.5,
                detail: 0.5
            })
            .boxed(),
        Building(Sphere)
            .transformed(Linear::scaling(10.0, 10.0, 10.0)
                .then_displacement_of(20.0, 20.0, 20.0))
            .with_outer_texture(Constant(Emissive(Color::grey_shade(16.0))))
            .boxed(),
    ]))
        .transformed(Translation::new(-0.35, -0.2, -distance))
        .path_traced()
        .with_environment(Sky)
        .with_depth(16)
        .done();
    let time = std::time::SystemTime::now();
    let image = camera.shoot(&world, 16 * 14, 16);
    println!("{:?}", time.elapsed());
    image.save("_image_2_2.png");
}
