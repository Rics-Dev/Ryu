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
struct Args {
    /// File to open (omit to start with a scratch buffer)
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,    
}



#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse(); // happens before tracing setup intentionally so --help/--version exit cleanly with no noise
    
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(ErrorLayer::default())
        .with(fmt::layer().pretty())
        .init();



    info!("Starting Ryu...");

    
    ryu_editor::run(args.file).await?;

    Ok(())
}