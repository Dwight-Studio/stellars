use std::collections::HashMap;
use eframe::egui::{vec2, InnerResponse, Popup, Sense, Ui};

pub enum MenuContent {
    Button { label: String },
    Separator,
}

#[derive(Default)]
pub struct MenuBar {
    being_used: bool,
}

impl MenuBar {
    pub fn ui(&mut self, ui: &mut Ui, menus: &HashMap<String, Vec<MenuContent>>, btn_clicked: impl Fn(String)) -> InnerResponse<()> {
        ui.horizontal(|ui| {
            for (menu_name, menu_content) in menus.iter() {
                let menu_btn = ui.menu_button(menu_name, |ui| {
                    menu_content.iter().for_each(|e| {
                        match e {
                            MenuContent::Button { label } => {
                                if ui.button(label).clicked() {
                                    btn_clicked(label.clone());
                                };
                            }
                            MenuContent::Separator => {
                                ui.separator();
                            }
                        }
                    })
                }).response;

                if menu_btn.context_menu_opened() {
                    self.being_used = true;
                }

                if self.being_used && !menu_btn.context_menu_opened() && menu_btn.hovered() {
                    Popup::close_all(ui.ctx());
                    Popup::open_id(ui.ctx(), Popup::from_response(&menu_btn).get_id());
                }
            }

            ui.set_min_size(vec2(ui.available_width(), ui.spacing().interact_size.y));
        })
    }
}