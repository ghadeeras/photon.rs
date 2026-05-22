use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::window::{Window, WindowAttributes};

pub trait AppFactory: Sized {

    type Output<'window>: App;

    fn window_attributes(&self, default_attributes: WindowAttributes) -> WindowAttributes {
        default_attributes
    }

    fn init<'window>(&mut self, window: Window) -> impl std::future::Future<Output = anyhow::Result<Self::Output<'window>>>;

}

pub trait App: Sized {

    fn window(&self) -> Arc<Window>;

    fn cleanup_resources(&mut self) {
    }

    fn resize(&mut self, size: PhysicalSize<u32>);

    fn redraw(&mut self);

}