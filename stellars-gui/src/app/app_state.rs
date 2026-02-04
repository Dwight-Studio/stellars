use crate::app::stellars_state::StellarsState;
use crate::widgets::config_window::ConfigWindow;

pub struct AppState {
    pub config_window_state: ConfigWindow,
    
    pub stellars_state: StellarsState,
}