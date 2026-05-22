use photon::win::boot;
use photon::wgpu::app::AppFactory;
use photon::wgpu::tracer::TracerFactory;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let bootstrapper = boot::Bootstrapper::new(AppFactory { renderer_factory: TracerFactory });
    bootstrapper.run()?;
    Ok(())
}