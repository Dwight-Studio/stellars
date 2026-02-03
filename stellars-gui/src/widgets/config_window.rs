use std::fmt::{Display, Formatter};
use eframe::egui::{ComboBox, Context, Grid, Window};

#[derive(PartialEq, Clone)]
enum VideoFormats {
    Ntsc,
    Pal,
    Secam
}

impl Display for VideoFormats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let displ = match self {
            VideoFormats::Ntsc => {"NTSC"}
            VideoFormats::Pal => {"PAL"}
            VideoFormats::Secam => {"SECAM"}
        };
        write!(f, "{}", displ)
    }
}

pub struct ConfigWindow {
    pub opened: bool,
    selected_value: VideoFormats,
}

impl ConfigWindow {
    pub fn show(ctx: &Context, config_window: &mut ConfigWindow) {
        let old_value = config_window.selected_value.clone();

        Window::new("Stellars Configuration")
            .open(&mut config_window.opened)
            .collapsible(false)
            .show(ctx, |ui| {
                Grid::new("config_grid")
                    .num_columns(2)
                    .spacing([50.0, 10.0])
                    .show(ui, |ui| {
                        ui.label("Video Format:");
                        ComboBox::from_id_salt("video_format")
                            .selected_text(format!("{}", config_window.selected_value))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut config_window.selected_value, VideoFormats::Ntsc, "NTSC");
                                ui.selectable_value(&mut config_window.selected_value, VideoFormats::Pal, "PAL");
                                ui.selectable_value(&mut config_window.selected_value, VideoFormats::Secam, "SECAM");
                            });
                        ui.end_row();
                    });
        });

        if old_value != config_window.selected_value {
            println!("Selected value {}", config_window.selected_value)
        }
    }
}

impl Default for ConfigWindow {
    fn default() -> Self {
        Self {
            opened: false,
            selected_value: VideoFormats::Ntsc,
        }
    }
}