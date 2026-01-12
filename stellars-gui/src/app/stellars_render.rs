use libstellars::{Color, Stellar, SCREEN_HEIGHT, SCREEN_WIDTH};
use pixels::{wgpu, Pixels, PixelsBuilder, SurfaceTexture};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use cpal::{FromSample, Sample, SampleRate, Stream, StreamError};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use winit::dpi::PhysicalSize;
use winit::event::ElementState;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;
use libstellars::controller::{Input, InputDevice, Joystick};
use libstellars::controller::Keypad::{R0C0, R0C1, R0C2, R1C0, R1C1, R1C2, R2C0, R2C1, R2C2, R3C0, R3C1, R3C2};
use crate::app::debugger_state::DebuggerState;

pub struct StellarsRender {
    pub window: Arc<Window>,
    render_buffer: Pixels<'static>,
    picture_buffer: Arc<RwLock<[Color; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize]>>,
    scale_factor: (u32, u32),
    target_framerate: f64,

    audio_stream: Option<Stream>,

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

        let mut audio_stream: Option<Stream> = None;
        if  let Some(device) = cpal::default_host().default_output_device() &&
            let Ok(mut configs) = device.supported_output_configs() &&
            let Some(config) = configs.next()
        {
            let sample_rate = config.with_max_sample_rate().sample_rate();
            let stellars = libstellars.clone();
            let stream = device.build_output_stream(
                &config.with_max_sample_rate().config(),
                move |data: &mut [u8], _: &cpal::OutputCallbackInfo| {
                    audio_callback(data, sample_rate, stellars.clone());
                },
                audio_error,
                None).expect("Output stream cannot be created.");
            stream.play().unwrap();

            audio_stream = Some(stream);
        } else {
            println!("No audio output device available.");
        }

        Self {
            window,
            render_buffer: pixels,
            picture_buffer: Arc::new(RwLock::new([Color { r: 0x00, g: 0x00, b: 0x00 }; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize])),
            scale_factor,
            target_framerate: 60.0,

            audio_stream,

            libstellars
        }
    }

    pub fn run(&mut self) {
        self.libstellars.read().unwrap().load_rom(PathBuf::from("./stellars-gui/resources/Tennis - Le Tennis (1981) (Activision, Alan Miller).bin"));

        let stellars = self.libstellars.clone();
        let picture_buffer = self.picture_buffer.clone();
        let window = self.window.clone();
        let target_framerate = self.target_framerate;

        std::thread::spawn(move || {
            let frame_duration = Duration::from_secs_f64(1.0 / target_framerate);
            let mut frame_start = Instant::now();
            let mut debugger_state = DebuggerState::new(stellars.clone());

            println!("For help, type \"help\".");
            loop {
                debugger_state.process_debugger_input();
                
                if debugger_state.redraw_requested() {
                    debugger_state.update();
                    picture_buffer.write().unwrap().copy_from_slice(stellars.read().unwrap().unsafe_get_picture_buffer().as_slice());
                    window.request_redraw();
                }

                while !debugger_state.is_paused() {
                    stellars.read().unwrap().execute();

                    debugger_state.update();

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

    pub fn update_inputs(&mut self, keycode: PhysicalKey, state: ElementState, input_device: InputDevice) {
        let pressed = state.is_pressed();

        let input: Input = match input_device {
            InputDevice::Keypad => {
                match keycode {
                    PhysicalKey::Code(KeyCode::Numpad7) => Input::Keypad(R0C0),
                    PhysicalKey::Code(KeyCode::Numpad8) => Input::Keypad(R0C1),
                    PhysicalKey::Code(KeyCode::Numpad9) => Input::Keypad(R0C2),
                    PhysicalKey::Code(KeyCode::Numpad4) => Input::Keypad(R1C0),
                    PhysicalKey::Code(KeyCode::Numpad5) => Input::Keypad(R1C1),
                    PhysicalKey::Code(KeyCode::Numpad6) => Input::Keypad(R1C2),
                    PhysicalKey::Code(KeyCode::Numpad1) => Input::Keypad(R2C0),
                    PhysicalKey::Code(KeyCode::Numpad2) => Input::Keypad(R2C1),
                    PhysicalKey::Code(KeyCode::Numpad3) => Input::Keypad(R2C2),
                    PhysicalKey::Code(KeyCode::Numpad0) => Input::Keypad(R3C0),
                    PhysicalKey::Code(KeyCode::NumpadComma) => Input::Keypad(R3C1),
                    PhysicalKey::Code(KeyCode::NumpadEnter) => Input::Keypad(R3C2),
                    _ => return,
                }
            },
            InputDevice::Joystick => {
                match keycode {
                    PhysicalKey::Code(KeyCode::ArrowRight) | PhysicalKey::Code(KeyCode::KeyD) => Input::Joystick(Joystick::Right), // Right
                    PhysicalKey::Code(KeyCode::ArrowLeft) | PhysicalKey::Code(KeyCode::KeyA) => Input::Joystick(Joystick::Left), // Left
                    PhysicalKey::Code(KeyCode::ArrowUp) | PhysicalKey::Code(KeyCode::KeyW) => Input::Joystick(Joystick::Up), // Up
                    PhysicalKey::Code(KeyCode::ArrowDown) | PhysicalKey::Code(KeyCode::KeyS) => Input::Joystick(Joystick::Down), // Down
                    PhysicalKey::Code(KeyCode::Enter) => Input::Joystick(Joystick::Button), // Button
                    _ => return,
                }
            }
        };

        self.libstellars.read().unwrap().update_inputs(input, pressed);
    }
}

fn audio_callback<T>(data: &mut [T], sample_rate: SampleRate, stellars: Arc<RwLock<Stellar>>)
    where T: Sample + FromSample<u8>
{
    let samples = stellars.read().unwrap().get_channel_1_samples(sample_rate as u64, data.len());

    for (sample_index, frame) in data.chunks_mut(1).enumerate() {
        for sample in frame.iter_mut() {
            *sample = T::from_sample((samples[sample_index] as f64 * 0.25) as u8);
        }
    }
}

fn audio_error(err: StreamError) {
    eprintln!("Audio error: {}", err);
}