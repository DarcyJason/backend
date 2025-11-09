use backend::{core::result::AppResult, run};

#[tokio::main]
async fn main() -> AppResult<()> {
    run().await
}
