use eframe::egui;
use crate::app::{App};

mod app;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native("Stellars", options, Box::new(|cc| Ok(Box::new(App::new(cc)))))
}
