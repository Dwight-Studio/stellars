use eframe::egui::{ComboBox, Context, Grid, Window};
use libstellars::VideoFormat;
use crate::app::app_state::AppState;

enum ConfigWindowEvent {
    VideoFormatChanged,
}

pub struct ConfigWindow {
    pub opened: bool,
    selected_value: VideoFormat,
}

impl ConfigWindow {
    pub fn show(ctx: &Context, state: &mut AppState) {
        let config_state = &mut state.config_window_state;
        let old_value = config_state.selected_value.clone();

        Window::new("Stellars Configuration")
            .open(&mut config_state.opened)
            .collapsible(false)
            .show(ctx, |ui| {
                Grid::new("config_grid")
                    .num_columns(2)
                    .spacing([50.0, 10.0])
                    .show(ui, |ui| {
                        ui.label("Video Format:");
                        ComboBox::from_id_salt("video_format")
                            .selected_text(format!("{}", config_state.selected_value))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut config_state.selected_value, VideoFormat::Ntsc, "NTSC");
                                ui.selectable_value(&mut config_state.selected_value, VideoFormat::Pal, "PAL");
                                ui.selectable_value(&mut config_state.selected_value, VideoFormat::Secam, "SECAM");
                            });
                        ui.end_row();
                    });
        });

        if old_value != config_state.selected_value {
            Self::config_window_event(state, ConfigWindowEvent::VideoFormatChanged);
        }
    }
    
    fn config_window_event(state: &mut AppState, config_event: ConfigWindowEvent) {
        match config_event { 
            ConfigWindowEvent::VideoFormatChanged => {state.stellars_state.set_video_format(state.config_window_state.selected_value.clone())} 
        }
    }
}

impl Default for ConfigWindow {
    fn default() -> Self {
        Self {
            opened: false,
            selected_value: VideoFormat::Ntsc,
        }
    }
}