use axum::Router;
use dotenvy::dotenv;
use std::sync::Arc;
use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;

use crate::{
    config::AppConfig,
    constants::logo::LOGO,
    core::{app_state::AppState, logger::logger},
    custom::result::AppResult,
    database::client::DBClient,
    routers::api_routers,
    utils::color::gradient_text,
};

pub async fn init_basics() -> AppResult<(WorkerGuard, Router, u16)> {
    let guard = logger();
    dotenv().ok();
    let _ = gradient_text(LOGO);
    let config = AppConfig::init()?;
    let db_client = DBClient::new(config.clone()).await?;
    let port = config.backend_server.backend_port;
    info!(
        "✅ The backend server is running at http://localhost:{}",
        port
    );
    let app_state = Arc::new(AppState::new(config, db_client));
    let router = api_routers(app_state.clone());
    Ok((guard, router, port))
}
