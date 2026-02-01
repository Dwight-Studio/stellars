use std::collections::BTreeMap;
use std::path::PathBuf;
use crate::app::stellars_render::StellarsRender;
use libstellars::controller::InputDevice;
use libstellars::Stellar;
use std::process::exit;
use std::sync::Arc;
use eframe::egui::{Context, Event, FontData, FontDefinitions, FontFamily, ViewportCommand};
use eframe::{egui, CreationContext, Frame};
use rfd::FileDialog;
use crate::app::stellars_audio::StellarsAudio;
use crate::app::stellars_state::StellarsState;
use crate::widgets::menu_bar::{MenuBar, MenuContent};

mod stellars_render;
mod debugger_state;
mod stellars_audio;
mod stellars_state;

#[derive(Clone)]
enum Menus {
    LoadRom,
    Quit,

    Configuration,
    Inputs,

    Website,
    Github,
    About,
}

pub const DEFAULT_FONT: &str = "cairopixel";

pub struct App {
    ctx: Context,

    stellars_render: StellarsRender,
    stellars_audio: StellarsAudio,
    stellars_state: StellarsState,
    input_device: InputDevice,

    menu_content: Vec<(String, Vec<MenuContent<Menus>>)>,
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        let libstellars = Stellar::new();
        let menu_content = Vec::from([
            (String::from("File"), vec![
                MenuContent::Button { btn: Menus::LoadRom, label: String::from("Load ROM") },
                MenuContent::Separator,
                MenuContent::Button { btn: Menus::Quit, label: String::from("Quit") },
            ]),
            (String::from("Emulation"), vec![
                MenuContent::Button { btn: Menus::Configuration, label: String::from("Configuration") },
                MenuContent::Button { btn: Menus::Inputs, label: String::from("Inputs") },
            ]),
            (String::from("Help"), vec![
                MenuContent::Button { btn: Menus::Website, label: String::from("Website") },
                MenuContent::Button { btn: Menus::Github, label: String::from("GitHub") },
                MenuContent::Button { btn: Menus::About, label: String::from("About") },
            ])
        ]);

        setup_fonts(&cc.egui_ctx);

        Self {
            ctx: cc.egui_ctx.clone(),

            stellars_render: StellarsRender::new(libstellars.clone(), cc.egui_ctx.clone()),
            stellars_audio: StellarsAudio::new(libstellars.clone()),
            stellars_state: StellarsState::new(libstellars),
            input_device: InputDevice::Joystick,

            menu_content
        }
    }

    fn menu_btn_clicked(&self, btn: Menus) {
        match btn {
            Menus::LoadRom => {
                let file = FileDialog::new()
                    .add_filter("ROM File", &["a26", "bin"])
                    .set_directory(std::env::home_dir().unwrap_or_default())
                    .pick_file();

                if let Some(path) = file {
                    self.stellars_state.run_rom(path);
                }
            }
            Menus::Quit => {
                self.ctx.send_viewport_cmd(ViewportCommand::Close);
            }
            Menus::Configuration => {}
            Menus::Inputs => {}
            Menus::Website => {}
            Menus::Github => {}
            Menus::About => {}
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
            MenuBar::default().ui(ui, &self.menu_content, |btn| self.menu_btn_clicked(btn));
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

fn setup_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();
    let mut fam   = BTreeMap::new();

    fonts.font_data.insert(
        DEFAULT_FONT.to_owned(),
        Arc::from(FontData::from_static(include_bytes!("../assets/Cairopixel.ttf")))
    );

    fam.insert(FontFamily::Name(Arc::from(DEFAULT_FONT.to_owned())), vec![DEFAULT_FONT.to_owned()]);
    fonts.families.append(&mut fam);

    ctx.set_fonts(fonts);
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