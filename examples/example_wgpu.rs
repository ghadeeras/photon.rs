use photon::basic::vectors::Vec3D;
use photon::geometries::Sphere;
use photon::transforms::{AffineTransformation, Linear};
use photon::wgpu::app::AppFactory;
use photon::wgpu::geometry::sphere::SphereParams;
use photon::wgpu::geometry::transform::TransformedGeometry;
use photon::wgpu::geometry::ParameterizedGeometry;
use photon::wgpu::tracer::TracerFactory;
use photon::win::boot;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let bootstrapper = boot::Bootstrapper::new(AppFactory {
        name: "WebGPU-Based Example",
        renderer_factory: TracerFactory {
            geometry: ParameterizedGeometry(
                TransformedGeometry {
                    geometry: Sphere,
                    transformation: Linear::scaling(1.0, 1.5, 0.75)
                        .then_rotation(&Vec3D::Z, f64::atan2(1.0, 1.0))
                        .then_displacement_of(2.0, 0.0, 0.0)
                },
                SphereParams {
                    latitudes: 16,
                    longitudes: 32,
                }
            )
        }
    });
    bootstrapper.run()?;
    Ok(())
}