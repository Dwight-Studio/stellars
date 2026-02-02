use crate::widgets::config_window::ConfigWindow;

#[derive(Default)]
pub struct AppState {
    pub open_config: bool,
    pub config_window_state: ConfigWindow,
}

