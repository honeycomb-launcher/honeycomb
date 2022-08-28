use crate::profile::Version;
use crate::profile::Brand;

pub const BRANDS: &[Brand] = &[
    Brand::Vanilla,
];

pub const VERSIONS: &[Version] = &[
    Version::Release { major: 1, minor: 19, patch: Some(2) },
    Version::Release { major: 1, minor: 19, patch: Some(1) },
    Version::Release { major: 1, minor: 19, patch: None },
];
