use std::path::Path;

use clap::Parser;
use colored::Colorize;
use config::Validate;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use tracing::{debug, event, info, span, Instrument, Level};

use crate::config::Config;

mod cli;
mod config;

// all hail our overlod,
const BANNER: &str = r#"
                 _          _                       _       _       
                | |        | |                     | |     | |      
  _ __ _   _ ___| |_ ______| |_ ___ _ __ ___  _ __ | | __ _| |_ ___ 
 | '__| | | / __| __|______| __/ _ \ '_ ` _ \| '_ \| |/ _` | __/ _ \
 | |  | |_| \__ \ |_       | ||  __/ | | | | | |_) | | (_| | ||  __/
 |_|   \__,_|___/\__|       \__\___|_| |_| |_| .__/|_|\__,_|\__\___|
                                             | |                    
                                             |_|                    
"#;

// main async entrypoint
#[tokio::main]
async fn main() -> eyre::Result<()> {
    // initialize colored eyre for better-looking panics
    color_eyre::install().unwrap();

    // parse CLI arguments
    let cli = cli::CustomCli::parse();

    // load configuration and validate
    let config_file_path = cli.config;
    let config = Figment::new()
        .merge(Toml::file(&config_file_path))
        .extract::<Config>()?;

    if let Err(e) = config.validate() {
        eprintln!("Configuration validation failed: {}", e);
        std::process::exit(1);
    } else {
        info!(
            "Configuration loaded successfully from file: {:?}",
            &config_file_path
        );
        debug!("{:#?}", config.redact());
    }

    // initialize tracing/logging
    let log_level = match config.general.log_level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_ansi(true)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_max_level(log_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // fancy banner, because why not?
    println!("{}", BANNER.bright_green().bold());

    // parse the CLI arguments
    match cli.subcmd {
        cli::SubCommand::Daemon(_) => {
            info!("Starting daemon...");
            tokio::spawn(async move { daemon().await });
        }
    }

    tokio::signal::ctrl_c().await?;
    Ok(())
}

#[tracing::instrument]
async fn daemon() -> eyre::Result<()> {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
    loop {
        interval.tick().await;
        info!("Daemon is running...");
    }
}
