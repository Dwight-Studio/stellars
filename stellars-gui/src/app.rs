use std::path::PathBuf;
use crate::app::stellars_render::StellarsRender;
use libstellars::controller::InputDevice;
use libstellars::Stellar;
use std::process::exit;
use eframe::egui::{Context, Event};
use eframe::{egui, CreationContext, Frame};
use rfd::FileDialog;
use crate::app::stellars_audio::StellarsAudio;
use crate::app::stellars_state::StellarsState;

mod stellars_render;
mod debugger_state;
mod stellars_audio;
mod stellars_state;

pub struct App {
    stellars_render: StellarsRender,
    stellars_audio: StellarsAudio,
    stellars_state: StellarsState,
    input_device: InputDevice
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        let libstellars = Stellar::new();
        Self {
            stellars_render: StellarsRender::new(libstellars.clone(), cc.egui_ctx.clone()),
            stellars_audio: StellarsAudio::new(libstellars.clone()),
            stellars_state: StellarsState::new(libstellars),
            input_device: InputDevice::Joystick
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        // Input handling
        ctx.input(|input_state| {
            for event in &input_state.events {
                if let Event::Key {key, pressed, .. } = event {
                    self.stellars_render.update_inputs(*key, *pressed, self.input_device);
                }
            }
        });

        // Rendering
        egui::CentralPanel::default().frame(egui::Frame {
            fill: ctx.style().visuals.window_fill,
            ..Default::default()
        }).show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open ROM").clicked() {
                        let file = FileDialog::new()
                            .add_filter("ROM File", &["a26", "bin"])
                            .set_directory(std::env::home_dir().unwrap_or_default())
                            .pick_file();

                        if let Some(path) = file {
                            self.stellars_state.run_rom(path);
                        }
                    }
                    if ui.button("Quit").clicked() {}
                });
                ui.menu_button("Emulation", |ui| {
                    if ui.button("Configuration").clicked() {}
                    if ui.button("Inputs").clicked() {}
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("Website").clicked() {}
                    if ui.button("Github repository").clicked() {}
                    if ui.button("About").clicked() {}
                });
            });
            ui.separator();
            self.stellars_render.render(ui, &self.stellars_state);
        });

        ctx.request_repaint();
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.stellars_audio.stop();
        self.stellars_state.shutdown();

        exit(0);
    }
}

fn load_image_from_path(path: &PathBuf) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::ImageReader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

fn get_asset_path(filename: &str) -> PathBuf {
    let candidates = vec![
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets").join(filename),

        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.join("assets").join(filename)))
            .unwrap_or_default(),

        PathBuf::from("assets").join(filename),
    ];
    
    for candidate in candidates {
        if candidate.exists() {
            return candidate;
        }
    }

    PathBuf::from("assets").join(filename)
}