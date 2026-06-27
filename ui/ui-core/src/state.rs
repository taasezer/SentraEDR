use std::sync::Mutex;
use ui_models::DashboardState;

pub struct ApplicationState {
    pub dashboard: Mutex<DashboardState>,
    // State lifecycle management occurs here rather than uncontrolled globals
}
