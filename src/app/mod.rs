use glium::winit::{self, application::ApplicationHandler, error::EventLoopError, event_loop::EventLoop};

pub struct App<S:Scene>{
    context : Context<S>,

    event_loop : EventLoop<()>
}

struct Context<S:Scene>{
    scene : S,
    scene_name : String,
}

impl<S:Scene> From<S> for Context<S> {
    fn from(scene: S) -> Self {
        Self { scene, scene_name: String::new() }
    }
}
#[derive(Debug)]
pub enum AppError {
    
    EventLoopError(EventLoopError)
}

impl<S:Scene> App<S>{
    pub fn build(scene:S)->Result<Self,AppError>{
        
        //setting the context
        let context = scene.into();

        //building the event loop
        let event_loop = EventLoop::builder().build().map_err(AppError::EventLoopError)?;

        Ok(Self{context,event_loop})

    }

    pub fn run(mut self)->Result<(), AppError>{
        self.event_loop.run_app(&mut self.context).map_err(AppError::EventLoopError)

    }
}

impl<S:Scene> ApplicationHandler for Context<S>{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.scene.resumed(event_loop);
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.scene.handle_window_event(&event);
    }

    fn device_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            device_id: winit::event::DeviceId,
            event: winit::event::DeviceEvent,
        ) {
        self.scene.handle_device_event(event_loop, device_id, event);
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, _event: ()) {
        self.scene.handle_user_event(event_loop);
    }
}

#[allow(unused)]
pub trait Scene{

    fn init(&mut self);
    
    fn draw_frame(&mut self);

    fn update(&mut self);

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop){

    }

    fn handle_window_event(
        &mut self,
        event: &glium::winit::event::WindowEvent,
    ) {/*Nothing*/}

    fn handle_device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        _event: winit::event::DeviceEvent,
    ) {}

    fn handle_user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn handle_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _cause: winit::event::StartCause,
    ) {}

    fn on_exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}
}