use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use pixels::{wgpu, Pixels, PixelsBuilder, SurfaceTexture};
use winit::event_loop::EventLoopProxy;
use winit::window::Window;
use libstellars::{Color, Stellar, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::app::StellarsEvent;

pub const SCALE_FACTOR: f32 = 5.0;

pub struct StellarsRender {
    pub window: Arc<Window>,
    render_buffer: Pixels<'static>,
    picture_buffer: Arc<RwLock<[Color; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize]>>,

    libstellars: Arc<RwLock<Stellar>>,
}

impl StellarsRender {
    pub fn new(window: Arc<Window>, libstellars: Arc<RwLock<Stellar>>) -> Self {
        let surface_texture = SurfaceTexture::new(
            libstellars::SCREEN_WIDTH as u32 * SCALE_FACTOR as u32,
            libstellars::SCREEN_HEIGHT as u32 * SCALE_FACTOR as u32,
            window.clone(),
        );
        let mut pixels = PixelsBuilder::new(
            libstellars::SCREEN_WIDTH as u32,
            libstellars::SCREEN_HEIGHT as u32,
            surface_texture,
        )
        .blend_state(wgpu::BlendState::REPLACE)
        .build()
        .unwrap();

        // Strange fix for blank window on Windows
        pixels.enable_vsync(false);

        Self {
            window,
            render_buffer: pixels,
            picture_buffer: Arc::new(RwLock::new([Color { r: 0x00, g: 0x00, b: 0x00 }; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize])),
            libstellars
        }
    }

    pub fn run(&mut self, event_loop_proxy: EventLoopProxy<StellarsEvent>) {
        self.libstellars.read().unwrap().load_rom(PathBuf::from("./stellars-gui/resources/kernel_01.bin"));
        let stellars = self.libstellars.clone();
        let picture_buffer = self.picture_buffer.clone();

        std::thread::spawn(move || {
            loop {
                stellars.read().unwrap().execute();

                if let Some(pic_buff) = stellars.read().unwrap().get_picture_buffer() {
                    picture_buffer.write().unwrap().copy_from_slice(pic_buff.as_slice());
                    if let Err(e) = event_loop_proxy.send_event(StellarsEvent::FrameReady) {
                        eprintln!("{e}");
                    }
                }
            }
        });
    }

    pub fn render(&mut self) {
        let mut buff = Vec::<u8>::new();

        for pixel in self.picture_buffer.read().unwrap().iter() {
            buff.push(pixel.r); // Red
            buff.push(pixel.g); // Green
            buff.push(pixel.b); // Blue
            buff.push(0xFF); // Alpha
        }

        let frame = self.render_buffer.frame_mut();
        frame[..buff.len()].copy_from_slice(buff.as_slice());

        self.render_buffer.render().unwrap();
    }
}