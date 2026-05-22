use crate::wgpu::gpu::GPU;
use std::sync::Arc;
use winit::window::Window;

pub struct Canvas<'window> {
    gpu_surface: wgpu::Surface<'window>,
    config: wgpu::SurfaceConfiguration,
    window: Arc<Window>,
}

impl<'window> Canvas<'window> {

    pub fn new(window: Window, gpu_instance: &wgpu::Instance) -> anyhow::Result<Self> {
        let window_ref = Arc::new(window);
        let gpu_surface = gpu_instance.create_surface(window_ref.clone())?;
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            width: window_ref.inner_size().width,
            height: window_ref.inner_size().height,
            present_mode: wgpu::PresentMode::AutoVsync,
            view_formats: vec![],
            alpha_mode: wgpu::CompositeAlphaMode::default(),
            desired_maximum_frame_latency: 1
        };
        Ok(Self {
            gpu_surface,
            config,
            window: window_ref,
        })
    }

    pub fn window(&self) -> Arc<Window> {
        self.window.clone()
    }

    pub fn surface(&self) -> &wgpu::Surface<'window> {
        &self.gpu_surface
    }

    pub fn preferred_format(&self) -> wgpu::TextureFormat {
        self.config.format
    }

    pub fn acquire_frame(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.gpu_surface.get_current_texture()
    }

    /// Reconfigure the surface using the current configuration.
    pub fn reconfigure(&mut self, gpu: &GPU) {
        let formats = &self.gpu_surface.get_capabilities(&gpu.adapter).formats;
        self.config.format = formats.iter()
            .find(|f| f.is_srgb())
            .unwrap_or(formats.get(0).unwrap_or(&wgpu::TextureFormat::Rgba8UnormSrgb))
            .clone();
        self.gpu_surface.configure(&gpu.device, &self.config);
    }

    pub fn resize(&mut self, gpu_device: &wgpu::Device, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.gpu_surface.configure(gpu_device, &self.config);
        }
    }

}