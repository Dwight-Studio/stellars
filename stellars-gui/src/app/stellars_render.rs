use std::sync::Arc;
use pixels::{wgpu, Pixels, PixelsBuilder, SurfaceTexture};
use pixels::wgpu::PresentMode;
use winit::window::Window;

pub const SCALE_FACTOR: f32 = 5.0;

pub struct StellarsRender {
    pub window: Arc<Window>,
    render_buffer: Pixels<'static>,
}

impl StellarsRender {
    pub fn new(window: Arc<Window>) -> Self {
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
        //.enable_vsync(false)
        .blend_state(wgpu::BlendState::REPLACE)
        .build()
        .unwrap();

        pixels.enable_vsync(false);

        Self { window, render_buffer: pixels }
    }

    pub fn render(&mut self) {
        let mut buff = Vec::<u8>::new();

        for _ in 0..libstellars::SCREEN_HEIGHT as u32 * libstellars::SCREEN_WIDTH as u32 {
            buff.push(0x00); // Red
            buff.push(0xFF); // Green
            buff.push(0x00); // Blue
            buff.push(0xFF); // Alpha
        }

        let frame = self.render_buffer.frame_mut();
        frame[..buff.len()].copy_from_slice(buff.as_slice());

        self.render_buffer.render().unwrap();
    }
}