use clap::Parser;
use log::{debug, info};
mod config;
mod handlers;
mod service;

/// Struct containing the command line arguments.
#[derive(Parser, Debug)]
pub struct Args {
    /// Path to the configuration file.
    ///
    /// # Short and long options
    ///
    /// -  `-c`, `--config`
    ///
    /// # Default value
    ///
    /// `config.yml`
    #[arg(short, long, default_value = "config.yml")]
    pub config: String,
}

/// The main entry point for the program.
#[tokio::main]
async fn main() {
    env_logger::init();
    // Parse command line arguments.
    info!("😀 application started");
    let args = Args::parse();
    debug!("😀 parsed command line arguments: {args:?}");
    match config::parse(args.config) {
        // If the configuration file is successfully parsed, start the service.
        Ok(cfg) => {
            service::listen(&cfg).await;
        }
        // If the configuration file  is not successfully parsed, panic.
        Err(err) => {
            panic!("{err}")
        }
    }
    info!("🥹riot rso example application completed successfully, goodbye.");
}
