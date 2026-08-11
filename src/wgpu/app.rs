use crate::wgpu::meshes::{MeshGenerator, Meshable};
use crate::wgpu::primitive_assembly::PrimitiveAssembly;
use crate::wgpu::tracer::{Tracer, Triangles};
use instant_app::core::app;
use instant_app::wgpu::canvas::Canvas;
use instant_app::wgpu::{gpu, init_wgpu};
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;
use winit::event_loop::{EventLoop, OwnedDisplayHandle};
use winit::window::{Window, WindowAttributes};

pub struct AppConfig<G: MeshGenerator, M: Meshable<Generator=G>> {
    pub name: &'static str,
    pub meshable: M,
    pub params: G::Params,
}
pub struct App<'window, G: MeshGenerator, M: Meshable<Generator=G>> {
    canvas: Canvas<'window>,
    renderer: Tracer,
    app_config: PhantomData<AppConfig<G, M>>,
}

pub trait Renderer : Sized {

    fn gpu(&self) -> &gpu::GPU;

    fn animate(&mut self, elapses_time: Duration);

    fn render(&self, texture: &wgpu::Texture);

}

impl<'window, G: MeshGenerator, M: Meshable<Generator=G>> App<'window, G, M> {

    pub fn run(config: AppConfig<G, M>) -> anyhow::Result<()> {
        instant_app::run::<App<G, M>>(config)
    }

}

impl<'window, G: MeshGenerator, M: Meshable<Generator=G>> app::EmptyApp for App<'window, G, M> {

    type Config = AppConfig<G, M>;

    async fn new(config: Self::Config, window: Arc<Window>, display: OwnedDisplayHandle) -> anyhow::Result<Self> {
        let canvas = init_wgpu(window, display).await?;
        let assembly = PrimitiveAssembly::new(&canvas.gpu);
        let generator = config.meshable.generator(&canvas.gpu);
        let mesh = generator.mesh(&config.params);
        let triangles_buffer = assembly.triangles(&mesh);
        let renderer = Tracer::new(canvas.gpu.clone(), canvas.preferred_format(), Triangles {
            triangles_buffer,
            vertices_buffer: mesh.vertices_buffer,
        });
        Ok(Self { app_config: PhantomData, canvas, renderer })
    }

    fn configure_loop(config: &Self::Config, _event_loop: &EventLoop<()>, default_attributes: WindowAttributes) -> WindowAttributes {
        default_attributes.with_title(config.name)
    }

    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.canvas.resize(size.width, size.height);
    }

    fn animate(&mut self, elapsed: Duration, _delta: Duration) {
        self.renderer.animate(elapsed);
    }

    fn redraw(&mut self) {
        // Try to acquire a frame; handle common surface errors gracefully.
        self.canvas.render_to_acquired_frame(|texture, _| {
            self.renderer.render(texture);
        });
    }

}