use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

pub struct Bootstrapper<T: AppFactory> {
    app_factory: T,
    app: Option<T::Output>,
}

impl<T: AppFactory> Bootstrapper<T> {

    pub fn new(app_factory: T) -> Self {
        Self { app_factory, app: None }
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop.run_app(&mut self).unwrap();
    }

}

impl<T: AppFactory> ApplicationHandler for Bootstrapper<T> {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = self.app_factory.window_attributes(Window::default_attributes());
        let window = event_loop.create_window(attributes).unwrap();
        let app = pollster::block_on(self.app_factory.init(window));
        self.app = Some(app);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        let app = self.app.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => {
                app.cleanup_resources();
                event_loop.exit()
            },
            WindowEvent::Resized(size) => {
                app.resize(size)
            },
            WindowEvent::RedrawRequested => {
                app.window().request_redraw();
                app.redraw();
            }
            _ => {}
        }
    }

}

pub trait AppFactory: Sized {

    type Output: App;

    fn window_attributes(&self, default_attributes: WindowAttributes) -> WindowAttributes {
        default_attributes
    }

    fn init(&mut self, window: Window) -> impl std::future::Future<Output = Self::Output>;

}

pub trait App: Sized {

    fn window(&self) -> Arc<Window>;

    fn cleanup_resources(&mut self) {
    }

    fn resize(&mut self, size: PhysicalSize<u32>);

    fn redraw(&mut self);

}
