use glium::{glutin::surface::WindowSurface, winit, Display};

pub trait ApplicationContext {
    fn draw_frame(&mut self, _display: &Display<WindowSurface>) {}
    fn new(display: &Display<WindowSurface>) -> Self;
    fn update(&mut self) {}
    fn handle_window_event(
        &mut self,
        _event: &glium::winit::event::WindowEvent,
        _window: &glium::winit::window::Window,
    ) {
    }

    fn handle_device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        _event: winit::event::DeviceEvent,
    ) {
        ()
    }

    fn handle_user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        ()
    }

    fn handle_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _cause: winit::event::StartCause,
    ) {
        ()
    }

    fn on_exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        ()
    }
}
