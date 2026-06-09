use photon::geometries::Sphere;
use photon::wgpu::app::AppFactory;
use photon::wgpu::geometry::sphere::Stacks;
use photon::wgpu::tracer::TracerFactory;
use photon::win::boot;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let bootstrapper = boot::Bootstrapper::new(AppFactory(TracerFactory {
        geometry: Sphere,
        params: Stacks(12)
    }));
    bootstrapper.run()?;
    Ok(())
}