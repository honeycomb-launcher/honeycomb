use std::path::PathBuf;
pub use clap::Parser;

use clap::Subcommand;

use crate::config;

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long, default_value = config::CONFIG_PATH_DEFAULT)]
    config: PathBuf,

    #[clap(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    pub const fn command(&self) -> &Option<Commands> {
        &self.command
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Profile {

        #[clap(long)]
        create: bool,
    }
}


pub mod theme {
    use std::fmt;
    use dialoguer::theme::Theme;

    pub struct HoneycombTheme;

    impl HoneycombTheme {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Theme for HoneycombTheme {
        /// Formats a prompt.
        #[inline]
        fn format_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
            write!(f, "> {}: ", prompt)
        }

        /// Formats out an error.
        #[inline]
        fn format_error(&self, f: &mut dyn fmt::Write, err: &str) -> fmt::Result {
            write!(f, "(!) {}", err)
        }

        /// Formats a confirm prompt.
        fn format_confirm_prompt(&self, f: &mut dyn fmt::Write, prompt: &str, default: Option<bool>) -> fmt::Result {
            if !prompt.is_empty() {
                write!(f, "> {} ", &prompt)?;
            }
            match default {
                None => write!(f, "yn? ")?,
                Some(true) => write!(f, "Yn? ")?,
                Some(false) => write!(f, "yN? ")?,
            }
            Ok(())
        }

        /// Formats a confirm prompt after selection.
        fn format_confirm_prompt_selection(&self, f: &mut dyn fmt::Write, prompt: &str, selection: Option<bool>) -> fmt::Result {
            let selection = selection.map(|b| if b { "Y" } else { "N" });

            match selection {
                Some(selection) if prompt.is_empty() => write!(f, "> {}", selection),
                Some(selection) => write!(f, "> {} {}", &prompt, selection),
                None if prompt.is_empty() => Ok(()),
                None => write!(f, "> {}", &prompt),
            }
        }

        /// Formats an input prompt.
        fn format_input_prompt(&self, f: &mut dyn fmt::Write, prompt: &str, default: Option<&str>) -> fmt::Result {
            match default {
                Some(default) if prompt.is_empty() => write!(f, "> [{}]: ", default),
                Some(default) => write!(f, "> {} [{}]: ", prompt, default),
                None => write!(f, "> {}: ", prompt),
            }
        }

        /// Formats an input prompt after selection.
        #[inline]
        fn format_input_prompt_selection(&self, f: &mut dyn fmt::Write, prompt: &str, sel: &str) -> fmt::Result {
            write!(f, "> {}: {}", prompt, sel)
        }

        /// Formats a password prompt.
        #[inline]
        fn format_password_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
            self.format_input_prompt(f, prompt, None)
        }

        /// Formats a password prompt after selection.
        #[inline]
        fn format_password_prompt_selection(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
            self.format_input_prompt_selection(f, prompt, "[hidden]")
        }

        /// Formats a select prompt.
        #[inline]
        fn format_select_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
            self.format_prompt(f, prompt)
        }

        /// Formats a select prompt after selection.
        #[inline]
        fn format_select_prompt_selection(&self, f: &mut dyn fmt::Write, prompt: &str, sel: &str) -> fmt::Result {
            self.format_input_prompt_selection(f, prompt, sel)
        }

        /// Formats a multi select prompt.
        #[inline]
        fn format_multi_select_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
            self.format_prompt(f, prompt)
        }

        /// Formats a sort prompt.
        #[inline]
        fn format_sort_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
            self.format_prompt(f, prompt)
        }

        /// Formats a multi_select prompt after selection.
        fn format_multi_select_prompt_selection(&self, f: &mut dyn fmt::Write, prompt: &str, selections: &[&str]) -> fmt::Result {
            write!(f, "> {}: ", prompt)?;
            for (idx, sel) in selections.iter().enumerate() {
                write!(f, "{}{}", if idx == 0 { "" } else { ", " }, sel)?;
            }
            Ok(())
        }

        /// Formats a sort prompt after selection.
        #[inline]
        fn format_sort_prompt_selection(&self, f: &mut dyn fmt::Write, prompt: &str, selections: &[&str]) -> fmt::Result {
            self.format_multi_select_prompt_selection(f, prompt, selections)
        }

        /// Formats a select prompt item.
        fn format_select_prompt_item(&self, f: &mut dyn fmt::Write, text: &str, active: bool) -> fmt::Result {
            write!(f, "{} {}", if active { ">" } else { "-" }, text)
        }

        /// Formats a multi select prompt item.
        fn format_multi_select_prompt_item(&self, f: &mut dyn fmt::Write, text: &str, checked: bool, active: bool) -> fmt::Result {
            write!(f, "{} {}", match (checked, active) {
                    (true, true) => "> [x]",
                    (true, false) => "  [x]",
                    (false, true) => "> [ ]",
                    (false, false) => "  [ ]",
                }, text)
        }

        /// Formats a sort prompt item.
        fn format_sort_prompt_item(&self, f: &mut dyn fmt::Write, text: &str, picked: bool, active: bool) -> fmt::Result {
            write!(f, "{} {}", match (picked, active) {
                    (true, true) => "> [x]",
                    (false, true) => "> [ ]",
                    (_, false) => "  [ ]",
                }, text)
        }
    }
}


