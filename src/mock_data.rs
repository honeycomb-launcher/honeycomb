// todo: this is all mock-data
// the plan is, that a server somewhere on the internet hosts
// this data

extern crate semver;
use semver::Version;

macro_rules! version {
    ($version:literal) => {
        semver::Version::parse($version).unwrap()
    };
}

use crate::profile::Brand;

pub const BRANDS: &[Brand] = &[
    Brand::Vanilla,
    Brand::Forge,
    Brand::Fabric,
    Brand::Quilt,
    Brand::OptiFine,
];

// this is a function because 'semver' cant handle being used in a const context
//
pub fn mock_minecraft_versions() -> Vec<Version> {
    vec![
        version!("1.19.2"),
        version!("1.19.1"),
        version!("1.19.0"),
        version!("1.18.2"),
        version!("1.18.1"),
        version!("1.18.0"),
        version!("1.17.1"),
        version!("1.16.5"),
        version!("1.16.4"),
        version!("1.16.3"),
        version!("1.16.2"),
        version!("1.16.1"),
        version!("1.16.0"),
        version!("1.15.2"),
        version!("1.15.1"),
        version!("1.15.0"),
        version!("1.14.4"),
        version!("1.14.3"),
        version!("1.14.2"),
        version!("1.14.1"),
        version!("1.14.0"),
        version!("1.13.2"),
        version!("1.13.1"),
        version!("1.13.0"),
    ]
}
