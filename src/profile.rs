use console::Term;
pub use serde::Serialize;
pub use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct Profile {
    /// Profile metadata
    meta: Meta,

    name: String,
    version: Version,
    brand: Brand,

}

#[derive(Serialize, Deserialize)]
struct Meta {
    /// # profile.meta.version
    /// The internal profile version
    version: u16, // if we ever get above 64k versions, wtf are we even doing xd
}

#[derive(Serialize, Deserialize)]
pub enum Version {
    Release {
        // the day, the game gets a recode, i'll
        major: u8,
        minor: u16,
        patch: Option<u16>,
    },
    ReleaseCandidate {
        major: u8,
        minor: u16,
        patch: Option<u16>,
        number: u8,
    },
    PreRelease {
        major: u8,
        minor: u16,
        patch: Option<u16>,
        number: u8,
    },
    Snapshot {
        year: u8,
        week: u8,
        /// While most snapshot versions generally have just a single letter as the suffix,
        /// some -- notably april fools snapshots -- often consist of one or more words
        suffix: String,
    },
    // todo: add old_beta & old_alpha; cbf rn since they dont use the same schema all the way thru
}

impl Version {
    pub fn readable(&self) -> String {
        match self {
            Self::Release { major, minor, patch } => {
                patch.map_or_else(|| format!("{major}.{minor}"), |patch| format!("{major}.{minor}.{patch}"))
            },
            Self::ReleaseCandidate { major, minor, patch, number } => {
                patch.map_or_else(|| format!("{major}.{minor}-rc{number}"), |patch| format!("{major}.{minor}.{patch}-rc{number}"))
            },
            Self::PreRelease { major, minor, patch, number } => {
                patch.map_or_else(|| format!("{major}.{minor}-pre{number}"), |patch| format!("{major}.{minor}.{patch}-pre{number}"))
            },
            Self::Snapshot { year, week, suffix } => {
                let suffix = suffix.clone();
                format!("{year}w{week}{suffix}")
            },
        }
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub enum Brand {
    Vanilla,
    Forge,
    Fabric,
    Quilt,
    OptiFine,
}

impl Brand {
    fn is_vanilla(&self) -> bool {
        *self == Brand::Vanilla
    }

    fn is_modded(&self) -> bool {
        !self.is_vanilla()
    }
}

impl Brand {
    fn readable(&self) -> String {
        match *self {
            Brand::Vanilla => String::from("vanilla"),
            Brand::Forge => String::from("forge"),
            Brand::Fabric => String::from("fabric"),
            Brand::Quilt => String::from("quilt"),
            Brand::OptiFine => String::from("optifine"),
        }
    }
}

pub fn create() -> Option<Profile> {
    let theme = &crate::cli::theme::HoneycombTheme::new();

    Term::stdout()
        .clear_screen()
        .unwrap();

    let brands: Vec<String> = crate::mock_data::BRANDS.iter()
        .map(|version| { version.readable() })
        .collect();

    let brand: usize = dialoguer::Select::with_theme(theme)
        .items(&brands)
        .default(0)
        .with_prompt("brand")
        .interact()
        .unwrap();

    let versions: Vec<String> = crate::mock_data::VERSIONS.iter()
        .map(|version| { version.readable() })
        .collect();

    let version: usize = dialoguer::Select::with_theme(theme)
        .items(&versions)
        .with_prompt("version")
        .default(0)
        .interact()
        .unwrap();

    let name: String = dialoguer::Input::with_theme(theme)
        .with_prompt("name")
        .with_initial_text(format!("{}-{}", brands[brand], versions[version]))
        .interact_text()
        .unwrap();

    None
}