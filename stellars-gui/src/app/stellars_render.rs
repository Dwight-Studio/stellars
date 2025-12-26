use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use pixels::{wgpu, Pixels, PixelsBuilder, SurfaceTexture};
use winit::window::Window;
use libstellars::Stellar;

pub const SCALE_FACTOR: f32 = 5.0;

pub struct StellarsRender {
    pub window: Arc<Window>,
    render_buffer: Pixels<'static>,

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

        Self { window, render_buffer: pixels, libstellars }
    }

    pub fn run(&mut self) {
        self.libstellars.read().unwrap().load_rom(PathBuf::from("./stellars-gui/resources/kernel_01.bin"));
        let stellars = self.libstellars.clone();

        std::thread::spawn(move || {
            loop {
                stellars.read().unwrap().execute();
            }
        });
    }

    pub fn render(&mut self) {
        if let Some(pic_buff) = self.libstellars.read().unwrap().get_picture_buffer() {
            let mut buff = Vec::<u8>::new();

            for pixel in pic_buff {
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
}