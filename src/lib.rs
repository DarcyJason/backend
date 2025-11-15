use std::net::{Ipv4Addr, SocketAddr};

use tokio::net::TcpListener;

use crate::{
    core::error::external::ExternalError,
    core::{init::init_app, result::AppResult},
    utils::shutdown::shutdown_signal,
};

pub mod config;
pub mod constants;
pub mod core;
pub mod database;
pub mod dto;
pub mod handlers;
pub mod middlewares;
pub mod models;
pub mod repositories;
pub mod routers;
pub mod services;
pub mod templates;
pub mod utils;
pub mod validation;

pub async fn run() -> AppResult<()> {
    let (_guard, router, port) = init_app().await?;
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
