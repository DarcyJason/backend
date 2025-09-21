use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_log() -> WorkerGuard {
    let file_appender = rolling::daily("logs/", "backend.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let console_layer = fmt::layer().with_target(false).with_ansi(true);
    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_target(false)
        .with_ansi(false);
    tracing_subscriber::registry()
        .with(EnvFilter::new("info"))
        .with(console_layer)
        .with(file_layer)
        .init();
    guard
}
