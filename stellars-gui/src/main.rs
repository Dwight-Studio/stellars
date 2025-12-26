use winit::event_loop::{ControlFlow, EventLoop};
use crate::app::{App, StellarsEvent};

mod app;

fn main() {
    let event_loop = EventLoop::<StellarsEvent>::with_user_event().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new(event_loop.create_proxy());
    let _ = event_loop.run_app(&mut app);
}
