use console::Term;
use serde::Serialize;
use serde::Deserialize;

extern crate semver;
use semver::Version;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    /// Profile metadata
    meta: Meta,

    name: String,
    version: Version,
    brand: Brand,

}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
struct Meta {
    /// # profile.meta.version
    /// The internal profile version
    version: u16, // if we ever get above 64k versions, wtf are we even doing xd
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Brand {
    Vanilla,
    Forge,
    Fabric,
    Quilt,
    OptiFine,
}

impl Brand {

    // see note in Brand::readable
    fn is_vanilla(self) -> bool {
        self == Self::Vanilla
    }

    // see note in Brand::readable
    fn is_modded(self) -> bool {
        !self.is_vanilla()
    }
}

impl Brand {
    // clippy told me to pass by value here
    // if it becomes a problem, feel free to change it
    fn readable(self) -> String {
        match self {
            Self::Vanilla => String::from("vanilla"),
            Self::Forge => String::from("forge"),
            Self::Fabric => String::from("fabric"),
            Self::Quilt => String::from("quilt"),
            Self::OptiFine => String::from("optifine"),
        }
    }

    fn from_readable(readable: &str) -> Option<Self> {
        match readable {
            "vanilla" => Some(Self::Vanilla),
            "forge" => Some(Self::Forge),
            "fabric" => Some(Self::Fabric),
            "quilt" => Some(Self::Quilt),
            "optifine" => Some(Self::OptiFine),
            _ => None,
        }
    }
}

pub fn create() -> Option<Profile> {
    let theme = &crate::cli::theme::HoneycombTheme::new();

    // i am only doing this here for now so clippy shuts the fuf up
    // extensive error checking must be done here! ill be surprised if users cant fuf up somehow
    // rember that user input is scary
    // todo: should the screen be cleared?
    if Term::stdout().clear_screen().is_err() {
        return None;
    }

    let brands: Vec<String> = crate::mock_data::BRANDS.iter()
        .map(|version| { version.readable() })
        .collect();

    let brand: usize = dialoguer::Select::with_theme(theme)
        .items(&brands)
        .default(0)
        .with_prompt("brand")
        .interact()
        .unwrap();

    let mut versions = vec!["latest".to_string()];

    crate::mock_data::mock_minecraft_versions()
        .iter()
        .map(|version| { version.to_string() })
        .for_each(|version| {
            versions.push(version);
        });

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



    // dont clutter the function with "useless" variables
    let version = {
        let version_index = if version == 0 { 0 } else { version - 1 };
        crate::mock_data::mock_minecraft_versions()[version_index].clone()
    };

    let profile = Profile {
        meta: Meta {
            version: 0
        },
        brand: crate::mock_data::BRANDS[brand],
        version,
        name,
    };

    Some(profile)
}
