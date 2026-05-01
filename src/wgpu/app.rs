use crate::wgpu::{boot, canvas, gpu, tracer};
use std::rc::Rc;

pub struct App<'a> {
    gpu: Rc<gpu::GPU>,
    canvas: canvas::Canvas<'a>,
    tracer: tracer::Tracer,
}

impl<'a> App<'a> {

    pub async fn new(window: &'a winit::window::Window) -> App<'a> {
        let mut canvas = canvas::Canvas::simple_new(window);
        let gpu_adapter = canvas.get_preferred_adapter().await;
        canvas.adjust_preferred_format(&gpu_adapter);
        let gpu = Rc::new(gpu::GPU::new(Rc::new(gpu_adapter)).await);
        let tracer = tracer::Tracer::new(gpu.clone(), canvas.preferred_format());
        Self { gpu, canvas, tracer }
    }

}

impl<'a> boot::App for App<'a> {
    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.canvas.resize(self.gpu.device().as_ref(), size.width, size.height);
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
                        self.canvas.reconfigure(self.gpu.device().as_ref());
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
                }
            }
        }
    }

}