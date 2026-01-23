use std::num::NonZeroU32;
use libstellars::{Color, Stellar, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use softbuffer::{Context, Surface};
use winit::dpi::PhysicalSize;
use winit::event::ElementState;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;
use libstellars::controller::{Console, Input, InputDevice, Joystick};
use libstellars::controller::Console::{P0DifficultyA, P0DifficultyB, P1DifficultyA, P1DifficultyB, Reset, Select, BW};
use libstellars::controller::Keypad::{R0C0, R0C1, R0C2, R1C0, R1C1, R1C2, R2C0, R2C1, R2C2, R3C0, R3C1, R3C2};
use crate::app::debugger_state::DebuggerState;

pub struct StellarsRender {
    pub window: Arc<Window>,
    render_surface: Surface<Arc<Window>, Arc<Window>>,
    picture_buffer: Arc<RwLock<[Color; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize]>>,
    scale_factor: (u32, u32),
    target_framerate: f64,

    libstellars: Arc<RwLock<Stellar>>,
}

impl StellarsRender {
    pub fn new(window: Arc<Window>, libstellars: Arc<RwLock<Stellar>>) -> Self {
        let ctx = Context::new(window.clone()).unwrap();
        let mut surface = Surface::new(&ctx, window.clone()).unwrap();
        let scale_factor = (window.inner_size().width / SCREEN_WIDTH, window.inner_size().height / SCREEN_HEIGHT);

        surface.resize(NonZeroU32::new(SCREEN_WIDTH * scale_factor.0).unwrap(), NonZeroU32::new(SCREEN_HEIGHT * scale_factor.1).unwrap()).unwrap();

        Self {
            window,
            render_surface: surface,
            picture_buffer: Arc::new(RwLock::new([Color { r: 0x00, g: 0x00, b: 0x00 }; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize])),
            scale_factor,
            target_framerate: 60.0,

            libstellars
        }
    }

    pub fn run(&mut self) {
        self.libstellars.read().unwrap().load_rom(PathBuf::from("./stellars-gui/resources/2k/3-D Tic-Tac-Toe.bin"));

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
        let mut buff = Vec::<u32>::new();
        let picture_buffer = self.picture_buffer.read().unwrap();
        let mut line_buffer = [Color { r: 0x00, g: 0x00, b: 0x00 }; SCREEN_WIDTH as usize];

        for i in 0..picture_buffer.len() {
            let pixel = picture_buffer[i];
            line_buffer[i % SCREEN_WIDTH as usize] = pixel;

            if i % SCREEN_WIDTH as usize == 159 {
                for _ in 0..self.scale_factor.1 {
                    for pixel in line_buffer {
                        for _ in 0..self.scale_factor.0 {
                            buff.push(((pixel.r as u32) << 16) | (pixel.g as u32) << 8 | pixel.b as u32);
                        }
                    }
                }
            }
        }

        drop(picture_buffer);

        if let Ok(mut frame) = self.render_surface.buffer_mut() {
            frame[..buff.len()].copy_from_slice(buff.as_slice());
        }

        self.render_surface.buffer_mut().unwrap().present().unwrap();
    }

    pub fn resize(&mut self, _: PhysicalSize<u32>) {
        self.scale_factor = (self.window.inner_size().width / SCREEN_WIDTH, self.window.inner_size().height / SCREEN_HEIGHT);
        self.render_surface.resize(NonZeroU32::new(SCREEN_WIDTH * self.scale_factor.0).unwrap(), NonZeroU32::new(SCREEN_HEIGHT * self.scale_factor.1).unwrap()).unwrap();
    }

    pub fn update_inputs(&mut self, keycode: PhysicalKey, state: ElementState, input_device: InputDevice) {
        let pressed = state.is_pressed();

        let mut input: Option<Input>;

        input = match keycode {
            PhysicalKey::Code(KeyCode::F1) => Some(Input::Console(Console::Color)),
            PhysicalKey::Code(KeyCode::F2) => Some(Input::Console(BW)),
            PhysicalKey::Code(KeyCode::F3) => Some(Input::Console(Select)),
            PhysicalKey::Code(KeyCode::F4) => Some(Input::Console(Reset)),
            PhysicalKey::Code(KeyCode::F5) => Some(Input::Console(P0DifficultyA)),
            PhysicalKey::Code(KeyCode::F6) => Some(Input::Console(P0DifficultyB)),
            PhysicalKey::Code(KeyCode::F7) => Some(Input::Console(P1DifficultyA)),
            PhysicalKey::Code(KeyCode::F8) => Some(Input::Console(P1DifficultyB)),
            _ => None,
        };

        if input.is_none() {
            input = match input_device {
                InputDevice::Keypad => {
                    match keycode {
                        PhysicalKey::Code(KeyCode::Numpad7) => Some(Input::Keypad(R0C0)),
                        PhysicalKey::Code(KeyCode::Numpad8) => Some(Input::Keypad(R0C1)),
                        PhysicalKey::Code(KeyCode::Numpad9) => Some(Input::Keypad(R0C2)),
                        PhysicalKey::Code(KeyCode::Numpad4) => Some(Input::Keypad(R1C0)),
                        PhysicalKey::Code(KeyCode::Numpad5) => Some(Input::Keypad(R1C1)),
                        PhysicalKey::Code(KeyCode::Numpad6) => Some(Input::Keypad(R1C2)),
                        PhysicalKey::Code(KeyCode::Numpad1) => Some(Input::Keypad(R2C0)),
                        PhysicalKey::Code(KeyCode::Numpad2) => Some(Input::Keypad(R2C1)),
                        PhysicalKey::Code(KeyCode::Numpad3) => Some(Input::Keypad(R2C2)),
                        PhysicalKey::Code(KeyCode::Numpad0) => Some(Input::Keypad(R3C0)),
                        PhysicalKey::Code(KeyCode::NumpadComma) => Some(Input::Keypad(R3C1)),
                        PhysicalKey::Code(KeyCode::NumpadEnter) => Some(Input::Keypad(R3C2)),
                        _ => None,
                    }
                },
                InputDevice::Joystick => {
                    match keycode {
                        PhysicalKey::Code(KeyCode::ArrowRight) | PhysicalKey::Code(KeyCode::KeyD) => Some(Input::Joystick(Joystick::Right)), // Right
                        PhysicalKey::Code(KeyCode::ArrowLeft) | PhysicalKey::Code(KeyCode::KeyA) => Some(Input::Joystick(Joystick::Left)), // Left
                        PhysicalKey::Code(KeyCode::ArrowUp) | PhysicalKey::Code(KeyCode::KeyW) => Some(Input::Joystick(Joystick::Up)), // Up
                        PhysicalKey::Code(KeyCode::ArrowDown) | PhysicalKey::Code(KeyCode::KeyS) => Some(Input::Joystick(Joystick::Down)), // Down
                        PhysicalKey::Code(KeyCode::Enter) => Some(Input::Joystick(Joystick::Button)), // Button
                        _ => None,
                    }
                }
            };
        }

        if let Some(input_value) = input {
            self.libstellars.read().unwrap().update_inputs(input_value, pressed);
        }
    }
}