pub const CONFIG_PATH_DEFAULT: &str = "~/.honeycomb.config.json";

use serde::Serialize;
use serde::Deserialize;

pub struct Config {
    launcher: Launcher,
}

struct Launcher {
    directory: String,
}