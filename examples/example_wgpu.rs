use photon::wgpu::app::AppFactory;
use photon::wgpu::tracer::TracerFactory;
use photon::win::boot;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let bootstrapper = boot::Bootstrapper::new(AppFactory(TracerFactory));
    bootstrapper.run()?;
    Ok(())
}