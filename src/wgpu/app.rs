use crate::wgpu::{canvas, gpu};
use crate::win::app;
use std::sync::Arc;
use std::time::Duration;
use winit::window::{Window, WindowAttributes};

pub struct AppFactory<F: RendererFactory> {
    pub name: &'static str,
    pub renderer_factory: F
}
pub struct App<'window, F: RendererFactory> {
    canvas: canvas::Canvas<'window>,
    renderer: F::Output,
}

pub trait RendererFactory : Sized {

    type Output: Renderer;

    fn new_renderer(&self, gpu: gpu::GPU, format: wgpu::TextureFormat) -> Self::Output;

}

pub trait Renderer : Sized {

    fn gpu(&self) -> &gpu::GPU;

    fn animate(&mut self, elapses_time: Duration);

    fn render(&self, texture: &wgpu::Texture);

}

impl<F: RendererFactory> app::AppFactory for AppFactory<F> {

    type Output<'window> = App<'window, F>;

    fn window_attributes(&self, default_attributes: WindowAttributes) -> WindowAttributes {
        default_attributes.with_title(self.name)
    }

    async fn init<'window>(&mut self, window: Window) -> anyhow::Result<Self::Output<'window>> {
        let gpu_instance = wgpu::Instance::new(&Default::default());
        let mut canvas = canvas::Canvas::new(window, &gpu_instance)?;
        let gpu = gpu::GPU::new(gpu_instance, Some(canvas.surface())).await?;
        canvas.reconfigure(&gpu);
        let renderer = self.renderer_factory.new_renderer(gpu, canvas.preferred_format());
        Ok(Self::Output { canvas, renderer })
    }

}

impl<'window, F: RendererFactory> app::App for App<'window, F> {

    fn window(&self) -> Arc<Window> {
        self.canvas.window()
    }

    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.canvas.resize(&self.renderer.gpu().device, size.width, size.height);
    }

    fn animate(&mut self, elapsed: Duration) {
        self.renderer.animate(elapsed);
    }

    fn redraw(&mut self) {
        // Try to acquire a frame; handle common surface errors gracefully.
        match self.canvas.acquire_frame() {
            Ok(texture) => {
                self.renderer.render(&texture.texture);
                texture.present();
            }
            Err(err) => {
                match err {
                    wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated => {
                        // Reconfigure with the current config and retry once
                        log::warn!("Surface error {:?}; reconfiguring and retrying frame", err);
                        self.canvas.reconfigure(&self.renderer.gpu());
                        if let Ok(texture) = self.canvas.acquire_frame() {
                            self.renderer.render(&texture.texture);
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
                    wgpu::SurfaceError::Other => {
                        log::error!("{:?}", err);
                    }
                }
            }
        }
    }

}