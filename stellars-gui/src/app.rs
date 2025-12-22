use std::process::exit;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop};
use winit::window::{Window, WindowId};
use crate::app::stellars_render::StellarsRender;

mod stellars_render;

pub struct App {
    stellars_render: Option<StellarsRender>,
}

impl App {
    pub fn new() -> Self {
        Self { stellars_render: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let stellars_render_attrs = Window::default_attributes()
            .with_title("Stellars Render")
            .with_inner_size(LogicalSize::new(
                640.0,
                480.0
            ));
        let render_window = Arc::new(event_loop.create_window(stellars_render_attrs).unwrap());
        let stellars_render = StellarsRender::new(render_window.clone());

        self.stellars_render = Some(stellars_render);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let stellars_render = match self.stellars_render.as_mut() {
            None => {return}
            Some(render) => render
        };

        match event {
            WindowEvent::CloseRequested => {
                if window_id == stellars_render.window.id() {
                    self.stellars_render = None;

                    event_loop.exit();
                    exit(0);
                }
            }
            WindowEvent::RedrawRequested => {
                if window_id == stellars_render.window.id() {
                    stellars_render.render();
                }
            }
            _ => {}
        }
    }
}