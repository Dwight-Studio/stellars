use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use libstellars::{Color, Stellar, SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct StellarsState {
    libstellars: Arc<RwLock<Stellar>>,

    target_framerate: f64,
    picture_buffer: Arc<RwLock<[Color; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize]>>,
    is_running: Arc<AtomicBool>,
    should_stop: Arc<AtomicBool>,
    should_run: Arc<AtomicBool>,
    emu_thread: Option<JoinHandle<()>>,
}

impl StellarsState {
    pub fn new(libstellars: Arc<RwLock<Stellar>>) -> StellarsState {
        let mut state = Self {
            libstellars,

            target_framerate: 50.0,
            picture_buffer: Arc::new(RwLock::new([Color { r: 0x00, g: 0x00, b: 0x00 }; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize])),
            is_running: Arc::new(AtomicBool::new(false)),
            should_stop: Arc::new(AtomicBool::new(false)),
            should_run: Arc::new(AtomicBool::new(false)),
            emu_thread: None,
        };

        state.run();
        state
    }

    pub fn picture_buffer(&self) -> Vec<Color> {
        if let Ok(buff) = self.picture_buffer.read() {
            buff.to_vec()
        } else {
            Vec::new()
        }
    }

    pub fn run_rom(&self, path: PathBuf) {
        if let Ok(libstellars) = self.libstellars.read() {
            libstellars.load_rom(path);
            self.should_run.store(true, Ordering::Relaxed);
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    pub fn shutdown(&mut self) {
        self.should_stop.store(true, Ordering::Relaxed);
        if let Some(th) = self.emu_thread.take() {
            th.join().expect("Emu thread panicked");
        }
    }

    fn run(&mut self) {
        let stellars = self.libstellars.clone();
        let picture_buffer = self.picture_buffer.clone();
        let target_framerate = self.target_framerate;
        let is_running = self.is_running.clone();
        let should_stop = self.should_stop.clone();
        let should_run = self.should_run.clone();

        self.emu_thread = Some(std::thread::spawn(move || {
            let frame_duration = Duration::from_secs_f64(1.0 / target_framerate);
            let mut frame_start = Instant::now();

            while !should_stop.load(Ordering::Relaxed) {
                if stellars.read().unwrap().rom_loaded() && should_run.load(Ordering::Relaxed) {
                    is_running.store(true, Ordering::Relaxed);
                    stellars.read().unwrap().execute();

                    if let Some(pic_buff) = stellars.read().unwrap().get_picture_buffer() {
                        picture_buffer.write().unwrap().copy_from_slice(pic_buff.as_slice());

                        let elapsed = frame_start.elapsed();
                        if elapsed < frame_duration {
                            std::thread::sleep(frame_duration - elapsed);
                        }
                        frame_start = Instant::now();
                    }
                } else {
                    is_running.store(false, Ordering::Relaxed);
                }
            }
        }));
    }
}