use ui_models::DashboardState;
use std::sync::Mutex;

pub struct ApplicationState {
    pub dashboard: Mutex<DashboardState>,
    // State lifecycle management occurs here rather than uncontrolled globals
}
