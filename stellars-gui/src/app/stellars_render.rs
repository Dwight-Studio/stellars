use libstellars::{Color, Stellar, SCREEN_HEIGHT, SCREEN_WIDTH};
use pixels::{wgpu, Pixels, PixelsBuilder, SurfaceTexture};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use winit::dpi::PhysicalSize;
use winit::event::ElementState;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

pub struct StellarsRender {
    pub window: Arc<Window>,
    render_buffer: Pixels<'static>,
    picture_buffer: Arc<RwLock<[Color; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize]>>,
    scale_factor: (u32, u32),
    target_framerate: f64,

    libstellars: Arc<RwLock<Stellar>>,
}

impl StellarsRender {
    pub fn new(window: Arc<Window>, libstellars: Arc<RwLock<Stellar>>) -> Self {
        let scale_factor = (window.inner_size().width / SCREEN_WIDTH, window.inner_size().height / SCREEN_HEIGHT);
        let surface_texture = SurfaceTexture::new(
            SCREEN_WIDTH * scale_factor.0,
            SCREEN_HEIGHT * scale_factor.1,
            window.clone(),
        );
        let mut pixels = PixelsBuilder::new(
            SCREEN_WIDTH * scale_factor.0,
            SCREEN_HEIGHT * scale_factor.1,
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
            scale_factor,
            target_framerate: 60.0,

            libstellars
        }
    }

    pub fn run(&mut self) {
        self.libstellars.read().unwrap().load_rom(PathBuf::from("./stellars-gui/resources/kernel_13.bin"));

        let stellars = self.libstellars.clone();
        let picture_buffer = self.picture_buffer.clone();
        let window = self.window.clone();
        let target_framerate = self.target_framerate;

        std::thread::spawn(move || {
            let frame_duration = Duration::from_secs_f64(1.0 / target_framerate);
            let mut frame_start = Instant::now();

            loop {
                stellars.read().unwrap().execute();

                if let Some(pic_buff) = stellars.read().unwrap().get_picture_buffer() {
                    picture_buffer.write().unwrap().copy_from_slice(pic_buff.as_slice());
                    window.request_redraw();

                    let elapsed = frame_start.elapsed();
                    if elapsed < frame_duration {
                        std::thread::sleep(frame_duration - elapsed);
                    }
                    frame_start = Instant::now();
                }
            }
        });
    }

    pub fn render(&mut self) {
        let mut buff = Vec::<u8>::new();
        let picture_buffer = self.picture_buffer.read().unwrap();
        let mut line_buffer = [Color { r: 0x00, g: 0x00, b: 0x00 }; SCREEN_WIDTH as usize];

        for i in 0..picture_buffer.len() {
            let pixel = picture_buffer[i];
            line_buffer[i % SCREEN_WIDTH as usize] = pixel;

            if i % SCREEN_WIDTH as usize == 159 {
                for _ in 0..self.scale_factor.1 {
                    for pixel in line_buffer {
                        for _ in 0..self.scale_factor.0 {
                            buff.push(pixel.r);
                            buff.push(pixel.g);
                            buff.push(pixel.b);
                            buff.push(0xFF);
                        }
                    }
                }
            }
        }

        drop(picture_buffer);

        let frame = self.render_buffer.frame_mut();
        frame[..buff.len()].copy_from_slice(buff.as_slice());

        self.render_buffer.render().unwrap();
    }

    pub fn resize(&mut self, _: PhysicalSize<u32>) {
        self.scale_factor = (self.window.inner_size().width / SCREEN_WIDTH, self.window.inner_size().height / SCREEN_HEIGHT);
        self.render_buffer.resize_surface(SCREEN_WIDTH * self.scale_factor.0, SCREEN_HEIGHT * self.scale_factor.1).unwrap();
        self.render_buffer.resize_buffer(SCREEN_WIDTH * self.scale_factor.0, SCREEN_HEIGHT * self.scale_factor.1).unwrap();
    }

    pub fn update_inputs(&mut self, keycode: PhysicalKey, state: ElementState) {
        let pressed = state.is_pressed();

        let (mask, button) = match keycode {
            PhysicalKey::Code(KeyCode::ArrowRight) | PhysicalKey::Code(KeyCode::KeyD) => (0b1000_0000, false), // Right
            PhysicalKey::Code(KeyCode::ArrowLeft) | PhysicalKey::Code(KeyCode::KeyA) => (0b0100_0000, false), // Left
            PhysicalKey::Code(KeyCode::ArrowUp) | PhysicalKey::Code(KeyCode::KeyW) => (0b0001_0000, false), // Up
            PhysicalKey::Code(KeyCode::ArrowDown) | PhysicalKey::Code(KeyCode::KeyS) => (0b0010_0000, false), // Down
            PhysicalKey::Code(KeyCode::Enter) => (0b1000_0000, true), // Button
            _ => return,
        };

        self.libstellars.read().unwrap().update_inputs(mask, pressed, button);
    }
}