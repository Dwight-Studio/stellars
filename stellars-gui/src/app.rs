use crate::app::stellars_render::StellarsRender;
use std::process::exit;
use std::sync::{Arc, RwLock};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
use winit::window::{Window, WindowId};
use libstellars::Stellar;

mod stellars_render;

pub enum StellarsEvent {
    FrameReady
}

pub struct App {
    libstellars: Arc<RwLock<Stellar>>,
    stellars_render: Option<StellarsRender>,
    event_loop_proxy: EventLoopProxy<StellarsEvent>,
}

impl App {
    pub fn new(event_loop_proxy: EventLoopProxy<StellarsEvent>) -> Self {
        Self {
            libstellars: Stellar::new(),
            stellars_render: None,
            event_loop_proxy,
        }
    }
}

impl ApplicationHandler<StellarsEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let stellars_render_attrs = Window::default_attributes()
            .with_title("Stellars Render")
            .with_inner_size(LogicalSize::new(
                640.0,
                480.0
            ));
        let render_window = Arc::new(event_loop.create_window(stellars_render_attrs).unwrap());
        let mut stellars_render = StellarsRender::new(render_window.clone(), self.libstellars.clone());
        stellars_render.run(self.event_loop_proxy.clone());

        self.stellars_render = Some(stellars_render);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let stellars_render = match self.stellars_render.as_mut() {
            None => {return}
            Some(render) => render
        };

        if event == WindowEvent::CloseRequested && window_id == stellars_render.window.id() {
            self.stellars_render = None;

            event_loop.exit();
            exit(0);
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: StellarsEvent) {
        let stellars_render = match self.stellars_render.as_mut() {
            None => {return}
            Some(render) => render
        };

        match event { StellarsEvent::FrameReady => {stellars_render.render()} }
    }
}