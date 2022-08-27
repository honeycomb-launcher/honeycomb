use serde::de::Unexpected::Str;
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
enum Version {
    Release {
        // the day, the game gets a recode, i'll
        major: u8,
        minor: u16,
        patch: u16,
    },
    ReleaseCandidate {
        release: Version::Release,
        number: u8,
    },
    PreRelease {
        release: Version::Release,
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

struct Brand(String);

impl From<&str> for Brand {
    fn from(s: &str) -> Self {
        Self(String::from(s))
    }
}

impl Brand {
    fn vanilla() -> Self {
        Self::from("vanilla")
    }
}

impl Brand {
    fn get(&self) -> String {
        self.0.clone()
    }
}