mod cli;

use cli::CLI;
use cli::Parser;

use cli::Commands;

fn main() {
    let cli: CLI = CLI::parse();

    match cli.command() {
        Some(Commands::Profile {}) => {
            println!("test :)");
        }
        None => {}
    }
}
