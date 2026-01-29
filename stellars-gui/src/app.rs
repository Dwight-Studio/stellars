use crate::app::stellars_render::StellarsRender;
use libstellars::controller::InputDevice;
use libstellars::Stellar;
use std::process::exit;
use eframe::egui::{Context, Event};
use eframe::{egui, CreationContext, Frame};
use crate::app::stellars_audio::StellarsAudio;

mod stellars_render;
mod debugger_state;
mod stellars_audio;

pub struct App {
    stellars_render: StellarsRender,
    stellars_audio: StellarsAudio,
    input_device: InputDevice
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        let libstellars = Stellar::new();
        let mut stellars_render = StellarsRender::new(libstellars.clone(), cc.egui_ctx.clone());
        stellars_render.run();
        Self {
            stellars_render,
            stellars_audio: StellarsAudio::new(libstellars),
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

        egui::CentralPanel::default().frame(egui::Frame {
            fill: ctx.style().visuals.window_fill,
            ..Default::default()
        }).show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open ROM").clicked() {}
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
            self.stellars_render.render(ui);
        });

        ctx.request_repaint();
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.stellars_audio.stop();

        exit(0);
    }
}