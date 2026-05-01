use crate::wgpu::gpu;
use std::rc::Rc;

pub struct Canvas<'a> {
    gpu_instance: Rc<wgpu::Instance>,
    gpu_surface: wgpu::Surface<'a>,
    config: wgpu::SurfaceConfiguration,
}

impl<'a> Canvas<'a> {

    pub fn simple_new(window: &'a winit::window::Window) -> Self {
        Self::new(window, Rc::new(gpu::GPU::new_instance()))
    }

    pub fn new(window: &'a winit::window::Window, gpu_instance: Rc<wgpu::Instance>) -> Self {
        let gpu_surface = gpu_instance.create_surface(window).unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            view_formats: vec![],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            desired_maximum_frame_latency: 1
        };
        Self {
            gpu_instance,
            gpu_surface,
            config,
        }
    }

    pub async fn get_preferred_adapter(&self) -> wgpu::Adapter {
        gpu::GPU::new_adapter(self.gpu_instance().as_ref(), Some(&self.gpu_surface)).await
    }

    pub fn adjust_preferred_format(&mut self, gpu_adapter: &wgpu::Adapter) {
        let formats = &self.gpu_surface.get_capabilities(&gpu_adapter).formats;
        self.config.format = formats.iter()
            .find(|f| f.is_srgb())
            .unwrap_or(formats.get(0).unwrap_or(&wgpu::TextureFormat::Rgba8UnormSrgb))
            .clone();
    }

    pub fn gpu_instance(&self) -> Rc<wgpu::Instance> {
        self.gpu_instance.clone()
    }

    pub fn preferred_format(&self) -> wgpu::TextureFormat {
        self.config.format
    }

    /// Try to acquire the next surface texture without panicking.
    pub fn acquire_frame(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.gpu_surface.get_current_texture()
    }

    /// Reconfigure the surface using the current configuration.
    pub fn reconfigure(&mut self, gpu_device: &wgpu::Device) {
        self.gpu_surface.configure(gpu_device, &self.config);
    }

    pub fn resize(&mut self, gpu_device: &wgpu::Device, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.gpu_surface.configure(gpu_device, &self.config);
    }

}