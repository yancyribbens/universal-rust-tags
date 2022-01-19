use serde::{Deserialize, Serialize};
use serde_json::Result;
use walkdir::WalkDir;
use std::ffi::OsString;
use std::path::PathBuf;


use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub manifest_path: String
}

impl Package {
    pub fn get_root(&self) -> Result<OsString> {
        let root;
        let mut path = PathBuf::from(&self.manifest_path);

        if path.pop() {
            root = path.into_os_string();
        } else {
            // TODO this should safely return an error
            // however in principle this should never happen
            panic!("oops!");
        }

        Ok(root)
    }

    pub fn get_package_files(self) -> Result<Vec<String>> {
        let mut result = Vec::new();
        let root = self.get_root().unwrap();

        for entry in WalkDir::new(&root)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok()) {

            let file = entry.file_name().to_string_lossy();
            
            if file.ends_with(".rs") {
                result.push(entry.path().to_string_lossy().into_owned());
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    use tempfile::tempdir;
    use std::fs::File;
    use std::io::{self, Write};

    #[test]
    fn test_package_get_root() {
        let package = Package {
            manifest_path: String::from("/path/Cargo.toml"),
        };

        let root = package.get_root().unwrap();

        assert_eq!(OsString::from("/path"), root);
    }

    #[test]
    fn test_get_files() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("lorem_ipsum.rs");
        let manifest = dir.path().join("Cargo.toml");

        File::create(&file_path).unwrap();
        File::create(&manifest).unwrap(); 

        let package = Package {
            manifest_path: manifest.into_os_string().into_string().unwrap(),
        };

        let files = package.get_package_files().unwrap();

        assert_eq!(files.len(), 1);
        assert_eq!(files[0], file_path.into_os_string().into_string().unwrap());
    }
}
