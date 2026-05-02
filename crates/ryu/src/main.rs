//! Ryu binary: argument parsing, config loading, initialization, and starting the main event loop.

use std::path::PathBuf;

use color_eyre::{Result, eyre::WrapErr}; // color-eyre — beautiful, context-rich error handling
use tracing::{info}; // Tracing — structured, contextual logging
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use tracing_error::ErrorLayer; // tracing-error — integrates with color-eyre for better error reporting in logs



use clap::Parser;

/// A terminal text editor
#[derive(Parser, Debug)]
#[command(name = "ryu", version, author, about)]
struct Cli {
    /// File to open
    file: Option<PathBuf>,
}



#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::parse(); // happens before tracing setup intentionally so --help/--version exit cleanly with no noise
    
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(ErrorLayer::default())
        .with(fmt::layer().pretty())
        .init();



    // Basic clap later, for now simple args
    info!("Starting Ryu...");


    // let contents = std::fs::read_to_string("../config.toml")
    //     .wrap_err("failed to read ryu config")?;
    
    // TODO: load config, init editor, run TUI
    ryu_editor::run().await?;

    Ok(())
}