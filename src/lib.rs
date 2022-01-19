use std::io::{Error};

pub mod packages;
pub mod package;

use crate::packages::Packages;

pub fn run() -> Result<(), Error> {
    let packages = Packages::new().packages;

    for p in packages {
        let files = p.get_package_files().unwrap();
        for f in files {
            println!("{}", &f);
        }
    }

    Ok(())
}
