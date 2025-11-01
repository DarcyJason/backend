use std::net::{Ipv4Addr, SocketAddr};

use tokio::net::TcpListener;

use crate::{
    core::{init::init_basics, shutdown::shutdown_signal},
    custom::{errors::external::ExternalError, result::AppResult},
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
    let (_guard, router, port) = init_basics().await?;
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
