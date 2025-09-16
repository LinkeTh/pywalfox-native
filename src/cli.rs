use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "pywalfox-native", about = "Linux-only native host for Pywalfox (Firefox)", version = version_string())]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Install the Firefox native messaging manifest (user scope)
    Install {
        /// Install manifest globally (requires root) [not implemented yet]
        #[arg(long)]
        global: bool,
    },
    /// Uninstall the Firefox native messaging manifest (user scope)
    Uninstall {
        /// Uninstall global manifest (requires root) [not implemented yet]
        #[arg(long)]
        global: bool,
    },
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

pub const fn version_string() -> &'static str {
    concat!("v", env!("CARGO_PKG_VERSION"))
}
