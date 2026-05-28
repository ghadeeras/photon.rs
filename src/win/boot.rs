use crate::win::app::{App, AppFactory};
use anyhow::Context;
use std::time::{Duration, SystemTime};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

pub struct Bootstrapper<'window, F: AppFactory> {
    app_factory: F,
    app: Option<F::Output<'window>>,
    initial_time: SystemTime,
}

impl<'window, T: AppFactory> Bootstrapper<'window, T> {

    pub fn new(app_factory: T) -> Self {
        Self { app_factory, app: None, initial_time: SystemTime::now() }
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop.run_app(&mut self)?;
        Ok(())
    }

    fn handle_event(event_loop: &ActiveEventLoop, event: WindowEvent, app: &mut T::Output<'window>, elapsed_time: Duration) {
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
                app.animate(elapsed_time);
                app.redraw();
            }
            _ => {}
        }
    }

}

impl<'window, T: AppFactory> ApplicationHandler for Bootstrapper<'window, T> {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = self.app_factory.window_attributes(Window::default_attributes());
        event_loop.create_window(attributes)
            .with_context(|| "Failed to create event loop window")
            .and_then(|window| {
                log::info!("Window created. Initializing the application ...");
                pollster::block_on(self.app_factory.init(window))
            })
            .and_then(|app| {
                log::info!("Application initialized.");
                self.app = Some(app);
                Ok(())
            })
            .unwrap_or_else(|e| {
                log::error!("Failed to initialize the application: {}", e);
                event_loop.exit();
            });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        if let Some(ref mut app) = self.app {
            Self::handle_event(event_loop, event, app, self.initial_time.elapsed().unwrap_or_else(|_| Duration::default()));
        } else {
            log::warn!("No application created yet to handle window events!");
        }
    }

}

