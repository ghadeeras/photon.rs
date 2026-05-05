use photon::wgpu::boot;
use photon::wgpu::app::AppFactory;

fn main() {
    env_logger::init();
    let bootstrapper = boot::Bootstrapper::new(AppFactory);
    bootstrapper.run();
}