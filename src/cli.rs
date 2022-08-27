pub use clap::Parser;

use clap::Subcommand;
use clap::clap_derive;

#[derive(Parser)]
pub struct CLI {

    #[clap(subcommand)]
    command: Option<Commands>,
}

impl CLI {
    pub(crate) fn command(&self) -> &Option<Commands> {
        &self.command
    }
}

#[derive(Subcommand)]
pub enum Commands {

    Profile {

    }
}

