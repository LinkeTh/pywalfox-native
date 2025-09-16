mod cli;
mod config;
mod daemon;
mod install;
mod native_messaging;
mod themes;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    if let Err(e) = real_main() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn real_main() -> Result<()> {
    // logging::init();
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Install { global: _ }) => {
            install::install_manifest(false)?;
        }
        Some(Commands::Uninstall { global: _ }) => {
            install::uninstall_manifest(false)?;
        }
        Some(Commands::Start) => {
            daemon::run()?;
        }
        Some(Commands::Update) => {
            themes::handle_update()?;
        }
        Some(Commands::Dark) => {
            themes::handle_dark()?;
        }
        Some(Commands::Light) => {
            themes::handle_light()?;
        }
        Some(Commands::Auto) => {
            themes::handle_auto()?;
        }
        None => {
            // No subcommand prints help by default via clap derive
            // But we can show version
            println!("{}", cli::version_string());
        }
    }

    Ok(())
}
