mod cli;
mod config;
mod daemon;
mod install;
mod native_messaging;
mod themes;

use crate::native_messaging::{send_colors, send_theme_mode};
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use tracing::{error, info};

fn main() {
    let file_appender = tracing_appender::rolling::never("/home/linket/", "pywalfox.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt().with_writer(non_blocking).init();

    if let Err(e) = real_main() {
        error!("error: {e}");
        std::process::exit(1);
    }
}

fn real_main() -> Result<()> {
    let cli = Cli::parse();
    info!("init pywalfox");

    match cli.command {
        Commands::Install => {
            install::install_manifest(false)?;
        }
        Commands::Uninstall => {
            install::uninstall_manifest(false)?;
        }
        Commands::Start => {
            daemon::run()?;
        }
        Commands::Update => {
            handle_update()?;
        }
        Commands::Dark => {
            handle_dark()?;
        }
        Commands::Light => {
            handle_light()?;
        }
        Commands::Auto => {
            handle_auto()?;
        }
    }

    Ok(())
}

fn handle_update() -> Result<()> {
    daemon::send_command("update")
    // send_colors()
}

fn handle_dark() -> Result<()> {
    send_theme_mode("dark")
}

fn handle_light() -> Result<()> {
    send_theme_mode("light")
}

fn handle_auto() -> Result<()> {
    send_theme_mode("auto")
}
