use clap::Parser;
use log::{debug, info};
mod config;
mod handlers;
mod service;

/// Struct containing the command line arguments.
///
/// This struct is defined using the `clap` crate to parse command line arguments. It specifically looks for
/// a configuration file path, which can be provided using `-c` or `--config` flags. If not provided, it defaults
/// to `config.yml`.
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
///
/// Initializes the logger, parses command line arguments using the `Args` struct, and attempts to parse
/// the configuration file specified by the command line arguments. If successful, it starts the service
/// with the parsed configuration. If the configuration file cannot be parsed, the application will panic.
///
/// Utilizes the `tokio` runtime for asynchronous operations, allowing the service to perform non-blocking
/// I/O operations.
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
