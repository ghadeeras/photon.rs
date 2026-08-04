use crate::wgpu::meshes::{MeshGenerator, Meshable};
use crate::wgpu::primitive_assembly::PrimitiveAssembly;
use crate::wgpu::tracer::{Tracer, Triangles};
use instant_app::core::app;
use instant_app::wgpu::canvas::Canvas;
use instant_app::wgpu::{gpu, init_wgpu};
use std::sync::Arc;
use std::time::Duration;
use winit::event_loop::OwnedDisplayHandle;
use winit::window::{Window, WindowAttributes};

pub struct AppFactory<G: MeshGenerator, M: Meshable<Generator=G>> {
    pub name: &'static str,
    pub meshable: M,
    pub params: G::Params,
}
pub struct App<'window> {
    canvas: Canvas<'window>,
    renderer: Tracer,
}

pub trait Renderer : Sized {

    fn gpu(&self) -> &gpu::GPU;

    fn animate(&mut self, elapses_time: Duration);

    fn render(&self, texture: &wgpu::Texture);

}

impl<'window, G: MeshGenerator, M: Meshable<Generator=G>> app::AppFactory<'window> for AppFactory<G, M> {

    type Output = App<'window>;

    fn window_attributes(&self, default_attributes: WindowAttributes) -> WindowAttributes {
        default_attributes.with_title(self.name)
    }

    async fn init(&mut self, window: Arc<Window>, display: OwnedDisplayHandle) -> anyhow::Result<Self::Output> {
        let canvas = init_wgpu(window, display).await?;
        let assembly = PrimitiveAssembly::new(&canvas.gpu);
        let generator = self.meshable.generator(&canvas.gpu);
        let mesh = generator.mesh(&self.params);
        let triangles_buffer = assembly.triangles(&mesh);
        let renderer = Tracer::new(canvas.gpu.clone(), canvas.preferred_format(), Triangles {
            triangles_buffer,
            vertices_buffer: mesh.vertices_buffer,
        });
        Ok(Self::Output { canvas, renderer })
    }

}

impl<'window> app::EmptyApp for App<'window> {

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