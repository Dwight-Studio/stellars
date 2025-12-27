use crate::app::stellars_render::StellarsRender;
use std::process::exit;
use std::sync::{Arc, RwLock};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{WindowEvent};
use winit::event_loop::{ActiveEventLoop};
use winit::window::{Window, WindowId};
use libstellars::{Stellar, SCREEN_HEIGHT, SCREEN_WIDTH};

mod stellars_render;

pub struct App {
    libstellars: Arc<RwLock<Stellar>>,
    stellars_render: Option<StellarsRender>,
}

impl App {
    pub fn new() -> Self {
        Self {
            libstellars: Stellar::new(),
            stellars_render: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let stellars_render_attrs = Window::default_attributes()
            .with_title("Stellars Render")
            .with_inner_size(LogicalSize::new(
                SCREEN_WIDTH as f64 * 4.0,
                SCREEN_HEIGHT as f64 * 2.0
            ));
        let render_window = Arc::new(event_loop.create_window(stellars_render_attrs).unwrap());
        let mut stellars_render = StellarsRender::new(render_window.clone(), self.libstellars.clone());
        stellars_render.run();

        self.stellars_render = Some(stellars_render);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let stellars_render = match self.stellars_render.as_mut() {
            None => {return}
            Some(render) => render
        };

        match event {
            WindowEvent::CloseRequested => {
                self.stellars_render = None;

                event_loop.exit();
                exit(0);
            }
            WindowEvent::RedrawRequested => {
                stellars_render.render();
            }
            WindowEvent::Resized(size) => {
                stellars_render.resize(size);
            }
            _ => {}
        }
    }
}