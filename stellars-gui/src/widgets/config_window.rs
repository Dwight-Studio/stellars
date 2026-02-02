use eframe::egui::{ComboBox, Context, Window};

#[derive(PartialEq, Clone, Debug)]
enum VideoFormats {
    Ntsc,
    Pal,
    Secam
}

pub struct ConfigWindow {
    selected_value: VideoFormats,
}

impl ConfigWindow {
    pub fn show(ctx: &Context, config_window: &mut ConfigWindow) {
        let old_value = config_window.selected_value.clone();

        Window::new("Stellars Configuration").show(ctx, |ui| {
            ComboBox::from_label("Video Format")
                .selected_text(format!("{:?}", config_window.selected_value))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut config_window.selected_value, VideoFormats::Ntsc, "NTSC");
                    ui.selectable_value(&mut config_window.selected_value, VideoFormats::Pal, "PAL");
                    ui.selectable_value(&mut config_window.selected_value, VideoFormats::Secam, "SECAM");
                });
        });

        if old_value != config_window.selected_value {
            println!("Selected value {:?}", config_window.selected_value)
        }
    }
}

impl Default for ConfigWindow {
    fn default() -> Self {
        Self {
            selected_value: VideoFormats::Ntsc,
        }
    }
}