use libstellars::{Color, Stellar, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};
use eframe::egui::{ColorImage, Context, Image, Key, TextureHandle, TextureOptions, Ui};
use eframe::emath::Vec2;
use libstellars::controller::{Console, Input, InputDevice, Joystick};
use libstellars::controller::Console::{P0DifficultyA, P0DifficultyB, P1DifficultyA, P1DifficultyB, Reset, Select, BW};
use libstellars::controller::Keypad::{R0C0, R0C1, R0C2, R1C0, R1C1, R1C2, R2C0, R2C1, R2C2, R3C0, R3C1, R3C2};
use crate::app::debugger_state::DebuggerState;
use crate::app::{get_asset_path, load_image_from_path};

pub struct StellarsRender {
    splash: ColorImage,
    picture_buffer: Arc<RwLock<[Color; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize]>>,
    texture: TextureHandle,
    target_framerate: f64,

    // FIXME: Emulator related stuff should be moved into another file
    is_running: Arc<AtomicBool>,

    libstellars: Arc<RwLock<Stellar>>,
}

impl StellarsRender {
    pub fn new(libstellars: Arc<RwLock<Stellar>>, ctx: Context) -> Self {
        let start_img =  {
            if let Ok(splash) = load_image_from_path(&get_asset_path("splash.jpg")) { splash }
            else { ColorImage::from_rgb([SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize], &vec![0; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize * 3]) }
        };

        Self {
            splash: start_img.clone(),
            picture_buffer: Arc::new(RwLock::new([Color { r: 0x00, g: 0x00, b: 0x00 }; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize])),
            texture: ctx.load_texture("render_texture", start_img, TextureOptions::NEAREST),
            target_framerate: 60.0,

            is_running: Arc::new(AtomicBool::new(false)),

            libstellars
        }
    }

    pub fn run(&mut self) {
        self.libstellars.read().unwrap().load_rom(PathBuf::from("./stellars-gui/resources/Tennis - Le Tennis (1981) (Activision, Alan Miller).bin"));

        let stellars = self.libstellars.clone();
        let picture_buffer = self.picture_buffer.clone();
        let target_framerate = self.target_framerate;
        let is_running = self.is_running.clone();

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
                }

                while !debugger_state.is_paused() {
                    is_running.store(true, std::sync::atomic::Ordering::Relaxed);
                    stellars.read().unwrap().execute();

                    debugger_state.update();

                    if let Some(pic_buff) = stellars.read().unwrap().get_picture_buffer() {
                        picture_buffer.write().unwrap().copy_from_slice(pic_buff.as_slice());

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

    pub fn render(&mut self, ui: &mut Ui) {
        let mut buff = Vec::<u8>::with_capacity(SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize * 3);
        let available_size = ui.available_size();

        if self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            let picture_buffer = self.picture_buffer.read().unwrap();

            for pixel in picture_buffer.iter() {
                buff.push(pixel.r);
                buff.push(pixel.g);
                buff.push(pixel.b);
            }

            drop(picture_buffer);

            self.texture.set(ColorImage::from_rgb([SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize], &buff), TextureOptions::NEAREST);
        } else {
            self.texture.set(self.splash.clone(), TextureOptions::LINEAR);
        }

        ui.add(
            Image::new(&self.texture).fit_to_exact_size(Vec2::new(available_size.x, available_size.y)).maintain_aspect_ratio(false)
        );
    }

    pub fn update_inputs(&mut self, key: Key, pressed: bool, input_device: InputDevice) {
        let mut input: Option<Input>;

        input = match key {
            Key::F1 => Some(Input::Console(Console::Color)),
            Key::F2 => Some(Input::Console(BW)),
            Key::F3 => Some(Input::Console(Select)),
            Key::F4 => Some(Input::Console(Reset)),
            Key::F5 => Some(Input::Console(P0DifficultyA)),
            Key::F6 => Some(Input::Console(P0DifficultyB)),
            Key::F7 => Some(Input::Console(P1DifficultyA)),
            Key::F8 => Some(Input::Console(P1DifficultyB)),
            _ => None
        };

        if input.is_none() {
            input = match input_device {
                InputDevice::Keypad => {
                    match key {
                        Key::Num7 => Some(Input::Keypad(R0C0)),
                        Key::Num8 => Some(Input::Keypad(R0C1)),
                        Key::Num9 => Some(Input::Keypad(R0C2)),
                        Key::Num4 => Some(Input::Keypad(R1C0)),
                        Key::Num5 => Some(Input::Keypad(R1C1)),
                        Key::Num6 => Some(Input::Keypad(R1C2)),
                        Key::Num1 => Some(Input::Keypad(R2C0)),
                        Key::Num2 => Some(Input::Keypad(R2C1)),
                        Key::Num3 => Some(Input::Keypad(R2C2)),
                        Key::Num0 => Some(Input::Keypad(R3C0)),
                        Key::Slash => Some(Input::Keypad(R3C1)), // FIXME: Numpad comma and enter cannot be mapped...
                        Key::Minus => Some(Input::Keypad(R3C2)),
                        _ => None
                    }
                },
                InputDevice::Joystick => {
                    match key {
                        Key::ArrowRight | Key::D => Some(Input::Joystick(Joystick::Right)),
                        Key::ArrowLeft  | Key::Q => Some(Input::Joystick(Joystick::Left)),
                        Key::ArrowUp    | Key::Z => Some(Input::Joystick(Joystick::Up)),
                        Key::ArrowDown  | Key::S => Some(Input::Joystick(Joystick::Down)),
                        Key::Enter               => Some(Input::Joystick(Joystick::Button)),
                        _ => None
                    }
                }
            };
        }

        if let Some(input_value) = input {
            self.libstellars.read().unwrap().update_inputs(input_value, pressed);
        }
    }
}