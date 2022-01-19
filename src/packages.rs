use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::process::Command;

use crate::package::Package;

#[derive(Serialize, Deserialize, Debug)]
pub struct Packages {
    pub packages: Vec<Package>
}

impl Packages {
    pub fn new() -> Packages {
        Packages::get_packages().unwrap()
    }

    pub fn get_packages() -> Result<Packages> {
        let output = Command::new("cargo")
            .arg("metadata")
            .arg("--format-version=1")
            .output()
            .expect("Failed to retrieve cargo metadata");

        let metadata: String = std::str::from_utf8(&output.stdout)
            .expect("Unable to convert Cargo Metadata to a string")
            .to_string();

        let p: Packages = serde_json::from_str(&metadata)
            .expect("Unable to create Package from json string");

        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn packages() -> Packages {
        let package_alpha = Package {
            manifest_path: String::from("/alpha/Cargo.toml"),
        };

        let package_beta = Package {
            manifest_path: String::from("/beta/Cargo.toml"),
        };

        let packages = vec![package_alpha, package_beta];

        let packages = Packages {
            packages: packages,
        };

        packages
    }

    #[test]
    fn test_package_serde() {
        let packages = packages();

        let j = serde_json::to_string(&packages).unwrap();
        let p: Packages = serde_json::from_str(&j).unwrap();
    }
}
