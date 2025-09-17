use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "pywalfox-native",
    about = "Linux-only native host for Pywalfox (Firefox)",
    version = "2.7.4"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Install the Firefox native messaging manifest (user scope)
    Install,
    /// Uninstall the Firefox native messaging manifest (user scope)
    Uninstall,
    /// Start the native host in the foreground (stdin/stdout)
    Start,
    /// Trigger an update (refetch colors)
    Update,
    /// Set theme mode to dark
    Dark,
    /// Set theme mode to light
    Light,
    /// Set theme mode to auto
    Auto,
}
