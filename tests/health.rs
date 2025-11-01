use std::sync::Arc;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use serde_json::json;

use axum_test::TestServer;
use backend::{
    config::AppConfig, core::app_state::AppState, custom::result::AppResult,
    database::client::DBClient, handlers::health::health_check,
};

#[tokio::test]
async fn is_healthy() -> AppResult<()> {
    dotenv().ok();
    let config = AppConfig::init()?;
    let db_client = DBClient::new(config.clone()).await?;
    let app_state = Arc::new(AppState::new(config, db_client));
    let router = Router::new()
        .route("/api/v1/health", get(health_check))
        .with_state(app_state);
    let server = TestServer::new(router).unwrap();
    let request = server.get(&"/api/v1/health");
    let response = request.await;
    response.assert_status_ok();
    response.assert_json(&json!({
      "success": {
        "code": 200,
        "message": "UP",
        "status": "OK"
      }
    }));
    Ok(())
}
