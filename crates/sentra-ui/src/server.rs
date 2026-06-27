use crate::{DashboardState, render_dashboard_html};
use axum::Router;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use std::sync::Arc;
use tokio::sync::RwLock;

/// The shared state passed to the web server handlers.
pub type SharedDashboardState = Arc<RwLock<DashboardState>>;

/// Configures and returns the Axum router for the SentraEDR dashboard.
pub fn dashboard_router(state: SharedDashboardState) -> Router {
    Router::new()
        .route("/", get(serve_dashboard_html))
        .with_state(state)
}

/// Handler for `GET /` - Returns the main HTML dashboard.
async fn serve_dashboard_html(State(state): State<SharedDashboardState>) -> impl IntoResponse {
    let dashboard = state.read().await;
    let html = render_dashboard_html(&dashboard);
    Html(html)
}
