use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use backend::{
    config::AppConfig, constants::logo::LOGO, cors::cors, custom::result::AppResult,
    database::client::DBClient, log::logger, routers::api_routers, shutdown::shutdown_signal,
    state::AppState, utils::color::gradient_text,
};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing::{error, info};

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv().ok();
    match gradient_text(LOGO) {
        Ok(_) => (),
        Err(e) => {
            error!("❌ Failed to initialize colorful logo: {}", e);
        }
    }
    let _guard = logger();
    let config = AppConfig::init().map_err(|e| {
        error!("❌ Failed to initialize config: {}", e);
        e
    })?;
    let db_client = DBClient::new(config.clone()).await?;
    let port = config.backend_server.backend_port;
    let frontend_address = config.frontend_server.frontend_address.clone();
    info!(
        "✅ The backend server is running at http://localhost:{}",
        port
    );
    let app_state = Arc::new(AppState::new(config, db_client));
    let router = api_routers()
        .layer(cors(frontend_address))
        .with_state(app_state);
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    let listener = TcpListener::bind(&address).await?;
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;
    Ok(())
}
