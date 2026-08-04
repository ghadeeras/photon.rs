use instant_app::run;
use photon::basic::vectors::Vec3D;
use photon::geometries::Sphere;
use photon::transforms::{AffineTransformation, Linear};
use photon::wgpu::app::AppFactory;
use photon::wgpu::meshes::sphere::SphereParams;
use photon::wgpu::meshes::transform::TransformedMeshable;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    run(AppFactory {
        name: "WebGPU-Based Example",
        meshable: vec![
            TransformedMeshable {
                meshable: Sphere,
                transformation: Linear::scaling(1.0, 1.5, 0.75)
                    .then_rotation(&Vec3D::Z, std::f64::consts::PI / 4.0)
                    .then_displacement_of(2.0, 0.0, 0.0)
            },
            TransformedMeshable {
                meshable: Sphere,
                transformation: Linear::scaling(1.0, 1.5, 0.75)
                    .then_displacement_of(-2.0, 0.0, 0.0)
                    .then_rotation(&Vec3D::Z, -std::f64::consts::PI / 4.0)
            },
        ],
        params: SphereParams {
            latitudes: 11,
            longitudes: 22,
        }
    })
}