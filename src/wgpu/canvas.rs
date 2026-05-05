use std::sync::Arc;
use winit::window::Window;

pub struct Canvas {
    gpu_instance: Arc<wgpu::Instance>,
    gpu_surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    window: Arc<Window>,
}

impl Canvas {

    pub fn simple_new(window: Arc<Window>) -> Self {
        Self::new(window, Arc::new(wgpu::Instance::new(&Default::default())))
    }

    pub fn new(window: Arc<Window>, gpu_instance: Arc<wgpu::Instance>) -> Self {
        let gpu_surface = gpu_instance.create_surface(window.clone()).unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::default(),
            view_formats: vec![],
            alpha_mode: wgpu::CompositeAlphaMode::default(),
            desired_maximum_frame_latency: 1
        };
        Self {
            gpu_instance,
            gpu_surface,
            config,
            window,
        }
    }

    pub async fn request_preferred_adapter(&self) -> wgpu::Adapter {
        self.gpu_instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&self.gpu_surface),
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
        }).await.unwrap()
    }

    pub fn adjust_preferred_format(&mut self, gpu_adapter: &wgpu::Adapter) {
        let formats = &self.gpu_surface.get_capabilities(&gpu_adapter).formats;
        self.config.format = formats.iter()
            .find(|f| f.is_srgb())
            .unwrap_or(formats.get(0).unwrap_or(&wgpu::TextureFormat::Rgba8UnormSrgb))
            .clone();
    }

    pub fn gpu_instance(&self) -> Arc<wgpu::Instance> {
        self.gpu_instance.clone()
    }

    pub fn window(&self) -> Arc<Window> {
        self.window.clone()
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