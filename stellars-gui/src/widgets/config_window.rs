use eframe::egui::{ComboBox, Context, Grid, Window};
use libstellars::VideoFormat;
use crate::app::app_state::AppState;

#[derive(Clone)]
pub struct ConfigWindow {
    pub opened: bool,

    sel_video_format: VideoFormat,
    hide_vblank: bool,
}

impl ConfigWindow {
    pub fn show(ctx: &Context, state: &mut AppState) {
        let config_state = &mut state.config_window_state;
        let old_value = config_state.clone();

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
                            .selected_text(format!("{}", config_state.sel_video_format))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut config_state.sel_video_format, VideoFormat::Ntsc, "NTSC");
                                ui.selectable_value(&mut config_state.sel_video_format, VideoFormat::Pal, "PAL");
                                ui.selectable_value(&mut config_state.sel_video_format, VideoFormat::Secam, "SECAM");
                            });
                        ui.end_row();

                        ui.label("Hide VBlank:");
                        ui.checkbox(&mut config_state.hide_vblank, "");
                    });
        });

        let curr_value = config_state.clone();
        if old_value.sel_video_format != curr_value.sel_video_format {
            state.stellars_state.set_video_format(state.config_window_state.sel_video_format.clone())
        }
        if old_value.hide_vblank != curr_value.hide_vblank {
            state.stellars_state.hide_vblank(curr_value.hide_vblank);
        }
    }
}

impl Default for ConfigWindow {
    fn default() -> Self {
        Self {
            opened: false,

            sel_video_format: VideoFormat::Ntsc,
            hide_vblank: false,
        }
    }
}