use crate::wgpu::{boot, canvas, gpu, tracer};
use std::sync::Arc;
use winit::window::Window;

pub struct AppFactory;
pub struct App {
    gpu: Arc<gpu::GPU>,
    canvas: canvas::Canvas,
    tracer: tracer::Tracer,
}

impl App {

    pub async fn new(window: Window) -> App {
        let w = Arc::new(window);
        let mut canvas = canvas::Canvas::simple_new(w.clone());
        let gpu_adapter = canvas.request_preferred_adapter().await;
        canvas.adjust_preferred_format(&gpu_adapter);
        let gpu = Arc::new(gpu::GPU::new(Arc::new(gpu_adapter)).await);
        canvas.reconfigure(gpu.device.as_ref());
        let tracer = tracer::Tracer::new(gpu.clone(), canvas.preferred_format());
        Self { gpu, canvas, tracer }
    }

}

impl boot::AppFactory for AppFactory {
    type Output = App;

    async fn init(&mut self, window: Window) -> Self::Output {
        App::new(window).await
    }

}

impl boot::App for App {

    fn window(&self) -> Arc<Window> {
        self.canvas.window()
    }

    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.canvas.resize(self.gpu.device.as_ref(), size.width, size.height);
    }

    fn redraw(&mut self) {
        // Try to acquire a frame; handle common surface errors gracefully.
        match self.canvas.acquire_frame() {
            Ok(texture) => {
                self.tracer.render(&texture.texture);
                texture.present();
            }
            Err(err) => {
                match err {
                    wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated => {
                        // Reconfigure with the current config and retry once
                        log::warn!("Surface error {:?}; reconfiguring and retrying frame", err);
                        self.canvas.reconfigure(self.gpu.device.as_ref());
                        if let Ok(texture) = self.canvas.acquire_frame() {
                            self.tracer.render(&texture.texture);
                            texture.present();
                        } else {
                            log::error!("Failed to acquire frame after reconfigure: {:?}", err);
                        }
                    }
                    wgpu::SurfaceError::Timeout => {
                        // Non-fatal; skip this frame
                        log::warn!("Surface acquisition timed out; skipping frame");
                    }
                    wgpu::SurfaceError::OutOfMemory => {
                        // Fatal; log and return (could also request app exit)
                        log::error!("Out of memory while acquiring surface frame");
                    }
                    _ => {
                        log::error!("{:?}", err);
                    }
                }
            }
        }
    }

}