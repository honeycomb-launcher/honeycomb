#![warn(clippy::all)] // the reddit comment was right, enabling more than just that really becomes annoying as hell

mod cli;
mod config;
mod profile;
mod mock_data;

use cli::Cli;
use cli::Commands;

use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command() {
        Some(Commands::Profile { create }) => {
            if *create {
                let profile = profile::create();
                println!("{:#?}", profile);
            }
        }
        None => {}
    }
}
