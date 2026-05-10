use tracing_appender::non_blocking::WorkerGuard;
use tracing_error::ErrorLayer; // tracing-error — integrates with color-eyre for better error reporting in logs
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing() -> WorkerGuard {
    // Log to ~/.local/share/ryu/ryu.log
    let log_dir  = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("ryu");
    std::fs::create_dir_all(&log_dir).ok();

    let file_appender = tracing_appender::rolling::daily(log_dir, "ryu.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info")))
        .with(ErrorLayer::default())
        .with(fmt::layer().with_writer(non_blocking)) // file only, no stdout
        .init();

    guard // must be held alive for the duration of main, dropped at end
}