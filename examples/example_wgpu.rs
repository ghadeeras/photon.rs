use photon::geometries::Sphere;
use photon::wgpu::app::AppFactory;
use photon::wgpu::geometry::sphere::Tessellation;
use photon::wgpu::tracer::TracerFactory;
use photon::win::boot;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let bootstrapper = boot::Bootstrapper::new(AppFactory {
        name: "WebGPU-Based Example",
        renderer_factory: TracerFactory {
            geometry: Sphere,
            params: Tessellation {
                latitudes: 16,
                longitudes: 32,
            }
        }
    });
    bootstrapper.run()?;
    Ok(())
}