pub struct Bootstrapper {
    event_loop: winit::event_loop::EventLoop<()>,
}

struct BootstrappedApp<'a, A: App> {
    app: A,
    window: &'a winit::window::Window,
}

impl Bootstrapper {

    pub fn new() -> (Self, winit::window::Window) {
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        let window = winit::window::WindowBuilder::new()
            .with_title("Photon.rs")
            .with_window_icon(Some(winit::window::Icon::from_rgba(vec![63, 127, 255, 255], 1, 1).unwrap()))
            .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
            .with_visible(false)
            .build(&event_loop)
            .unwrap();
        (Self { event_loop }, window)
    }

    pub fn run<A: App>(self, app: A, window: &winit::window::Window) {
        window.set_visible(true);
        let mut bootstrapped_app = BootstrappedApp { app, window };
        self.event_loop.run(move |event, target| {
            match event {
                winit::event::Event::Resumed => bootstrapped_app.window.request_redraw(),
                winit::event::Event::WindowEvent {
                    event: window_event,
                    ..
                } => bootstrapped_app.handle_window_event(window_event, target),
                _ => {}
            }
        }).unwrap();
    }

}

impl<'a, A: App> BootstrappedApp<'a, A> {

    fn handle_window_event(&mut self, window_event: winit::event::WindowEvent, target: &winit::event_loop::EventLoopWindowTarget<()>) {
        match window_event {
            winit::event::WindowEvent::CloseRequested => target.exit(),
            winit::event::WindowEvent::Resized(size) => self.app.resize(size),
            winit::event::WindowEvent::KeyboardInput { .. } => {}
            winit::event::WindowEvent::MouseInput { .. } => {}
            winit::event::WindowEvent::RedrawRequested => {
                self.window.request_redraw();
                self.app.redraw()
            },
            _ => {}
        }
    }

}

pub trait App {

    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>);

    fn redraw(&mut self);

}