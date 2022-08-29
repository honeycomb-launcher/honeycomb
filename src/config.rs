pub const CONFIG_PATH_DEFAULT: &str = "~/.honeycomb.config.json";

pub struct Config {
    launcher: Launcher,
}

struct Launcher {
    directory: String,
}
