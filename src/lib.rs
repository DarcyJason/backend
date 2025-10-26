use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing::{error, info};

use crate::{
    config::AppConfig,
    constants::logo::LOGO,
    core::{app_state::AppState, logger::logger, shutdown::shutdown_signal},
    custom::{errors::external::ExternalError, result::AppResult},
    database::client::DBClient,
    routers::api_routers,
    utils::color::gradient_text,
};

pub mod config;
pub mod constants;
pub mod core;
pub mod custom;
pub mod database;
pub mod dtos;
pub mod handlers;
pub mod lazy;
pub mod mail;
pub mod middlewares;
pub mod models;
pub mod repositories;
pub mod routers;
pub mod services;
pub mod utils;
pub mod validation;

pub async fn run() -> AppResult<()> {
    dotenv().ok();
    if let Err(e) = gradient_text(LOGO) {
        error!("❌ Failed to initialize colorful logo: {}", e);
    }
    let _guard = logger();
    let config = AppConfig::init().inspect_err(|e| {
        error!("❌ Failed to initialize config: {}", e);
    })?;
    let db_client = DBClient::new(config.clone()).await?;
    let port = config.backend_server.backend_port;
    info!(
        "✅ The backend server is running at http://localhost:{}",
        port
    );
    let app_state = Arc::new(AppState::new(config, db_client));
    let router = api_routers(app_state.clone());
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    let listener = TcpListener::bind(&address)
        .await
        .map_err(ExternalError::from)?;
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .map_err(ExternalError::from)?;
    Ok(())
}
