#![warn(
  clippy::all,
  clippy::pedantic,
  clippy::nursery,
  clippy::cargo,
)]

mod cli;
mod config;
mod profile;
mod mock_data;

use cli::Cli;
use cli::Parser;

use cli::Commands;

fn main() {
    let cli = Cli::parse();

    match cli.command() {
        Some(Commands::Profile { create }) => {
            if *create {
                profile::create();
            }
        }
        None => {}
    }
}
