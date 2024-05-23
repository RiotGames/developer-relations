use clap::Parser;
use log::{debug, info};
use url::{*};

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
    info!("😀 riot_rso_sample_app started");
    debug!("parsing command line arguments");

    let args = Args::parse();
    debug!("parsed command line arguments: {args:?}");
    match config::parse_file(args.config) {
        // If the configuration file is successfully parsed, start the service.
        Ok(cfg) => {
            info!("😁 starting");
            service::listen(&cfg);
            info!("🥹 stopped successfully , goodbye.");
        }
        // If the configuration file  is not successfully parsed, panic.
        Err(err) => {
            panic!("error {}", err)
        }
    }
}
