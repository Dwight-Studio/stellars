use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use libstellars::{Color, FormatDefinition, Stellar, VideoFormat};

pub struct StellarsState {
    libstellars: Arc<RwLock<Stellar>>,

    target_framerate: Arc<RwLock<f32>>,
    picture_buffer: Arc<RwLock<Vec<Color>>>,
    is_running: Arc<AtomicBool>,
    should_stop: Arc<AtomicBool>,
    should_run: Arc<AtomicBool>,
    emu_thread: Option<JoinHandle<()>>,
}

impl StellarsState {
    pub fn new(libstellars: Arc<RwLock<Stellar>>) -> StellarsState {
        let format = match libstellars.read() {
            Ok(libstellars) => { libstellars.curr_video_format() }
            Err(err) => { panic!("Error getting video format: {}", err); }
        };
        let mut state = Self {
            libstellars,

            target_framerate: Arc::new(RwLock::new(format.framerate())),
            picture_buffer: Arc::new(RwLock::new(vec![Color::default(); format.screen_width() as usize * format.screen_height() as usize])),
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

    pub fn video_format(&self) -> FormatDefinition {
        self.libstellars.read().unwrap_or_else(|e| e.into_inner()).curr_video_format()
    }

    pub fn run_rom(&self, path: PathBuf) {
        self.should_run.store(false, Ordering::Relaxed);
        while self.is_running.load(Ordering::Relaxed) {}
        if let Ok(libstellars) = self.libstellars.read() {
            libstellars.load_rom(path);
            self.should_run.store(true, Ordering::Relaxed);
        }
    }

    pub fn set_video_format(&self, video_format: VideoFormat) {
        self.should_run.store(false, Ordering::Relaxed);
        while self.is_running.load(Ordering::Relaxed) {}
        if let Ok(libstellars) = self.libstellars.read() {
            libstellars.set_video_format(video_format);
            let format = self.libstellars.read().unwrap().curr_video_format();
            self.picture_buffer.write().unwrap().resize(format.screen_width() as usize * format.screen_height() as usize, Color::default());
            *self.target_framerate.write().unwrap() = format.framerate();
            self.should_run.store(true, Ordering::Relaxed);
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    pub fn reset(&self) {
        self.stop();
        self.should_run.store(true, Ordering::Relaxed);
    }

    pub fn stop(&self) {
        self.should_run.store(false, Ordering::Relaxed);
        while self.is_running.load(Ordering::Relaxed) {}
        if let Ok(libstellars) = self.libstellars.read() {
            libstellars.reset();
        }
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
        let target_framerate = self.target_framerate.clone();
        let is_running = self.is_running.clone();
        let should_stop = self.should_stop.clone();
        let should_run = self.should_run.clone();

        self.emu_thread = Some(std::thread::spawn(move || {
            let mut old_framerate = *target_framerate.read().unwrap();
            let mut frame_duration = Duration::from_secs_f32(1.0 / old_framerate);
            let mut frame_start = Instant::now();

            while !should_stop.load(Ordering::Relaxed) {
                if should_run.load(Ordering::Relaxed) && stellars.read().unwrap().rom_loaded() {
                    if old_framerate != *target_framerate.read().unwrap() {
                        old_framerate = *target_framerate.read().unwrap();
                        frame_duration = Duration::from_secs_f32(1.0 / old_framerate);
                    }
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