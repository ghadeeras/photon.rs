use photon::wgpu::{app, boot};

fn main() {
    env_logger::init();
    pollster::block_on(request_app());
}

async fn request_app() {
    let (bootstrapper, window) = boot::Bootstrapper::new();
    let app = app::App::new(&window).await;
    bootstrapper.run(app, &window);
}