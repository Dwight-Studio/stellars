use std::sync::Arc;
use eframe::egui::{vec2, Color32, CornerRadius, FontId, InnerResponse, Margin, Popup, Response, Separator, Stroke, Style, TextStyle, Ui};
use eframe::egui::containers::menu::{MenuButton, MenuConfig};
use eframe::epaint::FontFamily;
use crate::app::DEFAULT_FONT;

pub enum MenuContent<N>
where
    N: Clone
{
    Button { btn: N, label: String },
    Separator,
}

#[derive(Default)]
pub struct MenuBar;

impl MenuBar {
    pub fn ui<N>(&mut self, ui: &mut Ui, menus: &Vec<(String, Vec<MenuContent<N>>)>, btn_clicked: impl Fn(N)) -> InnerResponse<()>
    where
        N: Clone
    {
        let mut context_opened = false;
        let mut resps: Vec<Response> = Vec::new();

        ui.horizontal(|ui| {
            for (menu_name, menu_content) in menus {
                menu_button_style(ui.style_mut());
                let menu_btn = MenuButton::new(menu_name).config(Self::menu_cfg()).ui(ui, |ui| {
                    menu_content.iter().for_each(|e| {
                        match e {
                            MenuContent::Button { btn, label } => {
                                if ui.button(label).clicked() {
                                    btn_clicked(btn.clone());
                                };
                            }
                            MenuContent::Separator => {
                                ui.add(Separator::default().spacing(0.0));
                            }
                        }
                    })
                }).0;

                if menu_btn.context_menu_opened() {
                    context_opened = true;
                }

                resps.push(menu_btn);
            }

            ui.set_min_size(vec2(ui.available_width(), ui.spacing().interact_size.y));

            for resp in resps {
                if context_opened && !resp.context_menu_opened() && resp.hovered() {
                    Popup::close_all(ui.ctx());
                    Popup::open_id(ui.ctx(), Popup::from_response(&resp).get_id());
                }
            }
        })
    }

    fn menu_cfg() -> MenuConfig {
        MenuConfig::new().style(menu_style)
    }
}

fn menu_style(style: &mut Style) {
    style.spacing.button_padding = vec2(20.0, 4.0);
    style.visuals.menu_corner_radius = CornerRadius::ZERO;
    style.spacing.menu_margin = Margin::ZERO;
    style.visuals.widgets.active.bg_stroke = Stroke::NONE;
    style.visuals.widgets.hovered.corner_radius = CornerRadius::ZERO;
    style.visuals.widgets.open.bg_stroke = Stroke::NONE;
    style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;
    style.visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    style.visuals.widgets.inactive.bg_stroke = Stroke::NONE;
    style.text_styles.insert(TextStyle::Button, FontId::new(20.0, FontFamily::Name(Arc::from(DEFAULT_FONT.to_owned()))));
}

fn menu_button_style(style: &mut Style) {
    style.spacing.button_padding = vec2(6.0, 4.0);
    style.visuals.widgets.active.bg_stroke = Stroke::NONE;
    style.visuals.widgets.active.weak_bg_fill = Color32::TRANSPARENT;
    style.visuals.widgets.open.bg_stroke = Stroke::NONE;
    style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;
    style.visuals.widgets.hovered.weak_bg_fill = Color32::TRANSPARENT;
    style.visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    style.visuals.widgets.inactive.bg_stroke = Stroke::NONE;
    style.text_styles.insert(TextStyle::Button, FontId::new(20.0, FontFamily::Name(Arc::from(DEFAULT_FONT.to_owned()))));
}