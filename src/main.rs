mod cli;
mod config;

use cli::CLI;
use cli::Parser;

use cli::Commands;

use config::Config;

fn main() {
    let cli = CLI::parse();
    //let config;

    match cli.command() {
        Some(Commands::Profile {}) => {
            println!("test :)");
        }
        None => {}
    }
}
