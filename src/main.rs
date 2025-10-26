use backend::{custom::result::AppResult, run};
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv().ok();
    run().await
}
