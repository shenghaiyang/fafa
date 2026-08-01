use anyhow::Context;
use std::path::PathBuf;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const APP_NAME_DIR: &str = "FaFa";
const LOGS_DIR: &str = "Logs";

fn logs_dir() -> anyhow::Result<PathBuf> {
    let dir = if cfg!(target_os = "macos") {
        dirs::home_dir()
            .with_context(|| "Could not find home directory")?
            .join("Library/Logs")
            .join(APP_NAME_DIR)
    } else {
        dirs::data_local_dir()
            .with_context(|| "Could not find data local directory")?
            .join(APP_NAME_DIR)
            .join(LOGS_DIR)
    };
    Ok(dir)
}

pub fn init_tracing() -> anyhow::Result<Vec<WorkerGuard>> {
    let logs_dir = logs_dir()?;
    if !std::fs::exists(&logs_dir)? {
        std::fs::create_dir_all(&logs_dir)?;
    }
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("fafa")
        .filename_suffix("log")
        .build(&logs_dir)?;
    let (file_non_locking, file_guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(file_non_locking)
        .with_ansi(false);
    let registry = tracing_subscriber::Registry::default().with(file_layer);
    let guards = if cfg!(debug_assertions) {
        let (std_non_blocking, std_guard) = tracing_appender::non_blocking(std::io::stdout());
        let stdout_layer = tracing_subscriber::fmt::layer()
            .with_writer(std_non_blocking)
            .with_target(true)
            .with_ansi(true);
        registry.with(stdout_layer).init();
        vec![file_guard, std_guard]
    } else {
        registry.init();
        vec![file_guard]
    };
    Ok(guards)
}
