//! Ryu binary: argument parsing, config loading, initialization, and starting the main event loop.

use std::{path::PathBuf};
use color_eyre::{Result}; // color-eyre — beautiful, context-rich error handling
use clap::Parser;
use ::tracing::info;



pub mod tracing;


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
    
    let _guard = tracing::init_tracing();


    info!("Starting Ryu...");

    
    ryu_editor::run(args.file).await?;

    Ok(())
}