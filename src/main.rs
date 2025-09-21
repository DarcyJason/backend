use backend::config::Config;
use backend::constants::logo::LOGO;
use backend::custom::result::AppResult;
use backend::database::client::DBClient;
use backend::observability::log::init_log;
use backend::routes::create_routes;
use backend::security::cors::cors;
use backend::state::AppState;
use dotenvy::dotenv;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;
use tracing::{error, info};

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv().ok();
    let _guard = init_log();
    println!("{}", LOGO);
    let config = Config::init().map_err(|e| {
        error!("Failed to initialize config: {}", e);
        e
    })?;
    let db_client = DBClient::new(config.clone()).await?;
    let port = config.backend_server.backend_port;
    let frontend_address = config.frontend_server.frontend_address.clone();
    info!("The backend server is running at http://localhost:{}", port);
    let app_state = Arc::new(AppState::new(config, db_client));
    let router = create_routes(app_state).layer(cors(frontend_address));
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    info!(
        "The classic API doc is at http://localhost:{}/swagger-ui",
        address.port()
    );
    info!(
        "The brief API doc is at http://localhost:{}/redoc",
        address.port()
    );
    info!(
        "The API doc bases on web component is at http://localhost:{}/rapidoc",
        address.port()
    );
    info!(
        "The modern API doc is at http://localhost:{}/scalar",
        address.port()
    );
    let listener = TcpListener::bind(&address).await?;

    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    tokio::select! {
        _ = ctrl_c => {},
    }
    println!();
    info!("Signal received, starting graceful shutdown");
}
