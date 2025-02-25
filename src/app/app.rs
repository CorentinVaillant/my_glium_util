use glium::{
    glutin::surface::WindowSurface,
    winit::{self, application::ApplicationHandler, error::EventLoopError},
};

use super::ApplicationContext;

pub struct State<T> {
    pub display: glium::Display<WindowSurface>,
    pub window: glium::winit::window::Window,
    pub context: T,
}

pub struct App<T> {
    state: Option<State<T>>,
    app_name: &'static str,
}

impl<T> App<T> {
    pub fn new(app_name: &'static str) -> Self {
        App {
            state: None,
            app_name,
        }
    }
}

#[derive(Debug)]
pub enum StateError {
    EventLoopError(EventLoopError),
}

impl<T: ApplicationContext> State<T> {
    pub fn new(event_loop: &glium::winit::event_loop::ActiveEventLoop, window_title: &str) -> Self {
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title(window_title)
            .build(event_loop);

        Self::from_display_window(display, window)
    }

    pub fn from_display_window(
        display: glium::Display<WindowSurface>,
        window: glium::winit::window::Window,
    ) -> Self {
        let context = T::new(&display);
        Self {
            display,
            window,
            context,
        }
    }
    pub fn run() -> Result<(), StateError> {
        let event_loop = glium::winit::event_loop::EventLoop::builder()
            .build()
            .map_err(StateError::EventLoopError)?;
        let mut app = App::<T> {
            state: None,
            app_name: "My App", //TODO
        };

        let result = event_loop.run_app(&mut app);
        result.map_err(StateError::EventLoopError)
    }
}

impl<T: ApplicationContext> ApplicationHandler<()> for App<T> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("[app handeler] : resumed");
        self.state = Some(State::new(event_loop, self.app_name));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            //---Resize---
            glium::winit::event::WindowEvent::Resized(new_size) => {
                if let Some(state) = &mut self.state {
                    state.display.resize(new_size.into());
                }
            }

            //---RedrawRequested---
            glium::winit::event::WindowEvent::RedrawRequested => {
                if let Some(state) = &mut self.state {
                    state.context.update();
                    state.context.draw_frame(&state.display);
                }
            }

            //---CloseRequested---
            glium::winit::event::WindowEvent::CloseRequested
            | glium::winit::event::WindowEvent::KeyboardInput {
                event:
                    glium::winit::event::KeyEvent {
                        state: glium::winit::event::ElementState::Pressed,
                        logical_key:
                            glium::winit::keyboard::Key::Named(glium::winit::keyboard::NamedKey::Escape),
                        ..
                    },
                ..
            } => {
                event_loop.exit();
                std::process::exit(0);
            }
            _ => (),
        }
        if let Some(state) = &mut self.state {
            state.context.handle_window_event(&event, &state.window);
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(state) = &self.state {
            state.window.request_redraw();
        }
    }

    fn device_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if let Some(state) = &mut self.state {
            state
                .context
                .handle_device_event(event_loop, device_id, event);
        }
    }

    fn new_events(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        if let Some(state) = &mut self.state {
            state.context.handle_event(event_loop, cause);
        }
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, _: ()) {
        if let Some(state) = &mut self.state {
            state.context.handle_user_event(event_loop);
        }
    }

    fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(state) = &mut self.state {
            state.context.on_exiting(event_loop);
        }
    }
}
