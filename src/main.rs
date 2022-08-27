mod cli;
mod config;

use std::fmt::Pointer;
use dialoguer::Select;

use cli::CLI;
use cli::Parser;

use cli::Commands;

use config::Config;
use crate::cli::theme::HoneycombTheme;

fn main() {
    let cli = CLI::parse();

    match cli.command() {
        Some(Commands::Profile {}) => {}
        None => {}
    }
}
