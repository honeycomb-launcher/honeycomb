use std::path::PathBuf;
pub use clap::Parser;

use clap::Subcommand;
use clap::clap_derive;

use crate::config;

#[derive(Parser)]
pub struct CLI {

    #[clap(short, long, default_value = config::CONFIG_PATH_DEFAULT)]
    config: PathBuf,

    #[clap(subcommand)]
    command: Option<Commands>,
}

impl CLI {
    pub fn command(&self) -> &Option<Commands> {
        &self.command
    }
}

#[derive(Subcommand)]
pub enum Commands {

    Profile {

    }
}

