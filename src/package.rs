use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageFile {
    pub(crate) version: String,
    pub dependencies: HashMap<String, String>,
    checksum: String,
}


impl Clone for PackageFile {
    fn clone(&self) -> Self {
        PackageFile {
            version: self.version.clone(),
            dependencies: self.dependencies.clone(),
            checksum: self.checksum.clone(),
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Packages {
    packages: HashMap<String, PackageFile>,
}

impl Packages {
    pub fn new() -> Self {
        Packages {
            packages: HashMap::new(),
        }
    }

    pub fn add_package(&mut self, name: String, package: PackageFile) {
        self.packages.insert(name, package);
    }

    pub fn remove_package(&mut self, name: String) {
        self.packages.remove(&name);
    }

    pub fn get_package(&self, name: String) -> Option<&PackageFile> {
        self.packages.get(&name)
    }

    pub fn replace_package(&mut self, name: String, package: PackageFile) {
        self.packages.remove(&name);
        self.packages.insert(name, package);
    }

    pub fn has_package(&mut self, name: String) -> bool {
        // check to see if name is in hashmap
        self.packages.contains_key(name.as_str())
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let mut file = File::create("/etc/uspm/packages.json")?;
        let serialized = serde_json::to_string_pretty(self)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn load(&mut self) -> Result<(), std::io::Error> {
        let mut file = File::open("/etc/uspm/packages.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let packages: Packages = serde_json::from_str(&contents)?;
        self.packages = packages.packages;
        Ok(())
    }
}

impl PackageFile {
    pub fn load(path: String) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let package: PackageFile = serde_json::from_str(&contents)?;
        Ok(package)
    }
}

pub fn compare_versions(a: String, b: String) -> i8 {
    // versions use semantic versioning ex: 1.2.3

    // split the string into its 3 parts and check each part
    let a_split: Vec<&str> = a.split(".").collect();

    let b_split: Vec<&str> = b.split(".").collect();

    // check major version
    if a_split[0].parse::<i32>().unwrap() > b_split[0].parse::<i32>().unwrap() {
        return 1;
    } else if a_split[0].parse::<i32>().unwrap() < b_split[0].parse::<i32>().unwrap() {
        return -1;
    }

    // check minor version
    if a_split[1].parse::<i32>().unwrap() > b_split[1].parse::<i32>().unwrap() {
        return 1;
    } else if a_split[1].parse::<i32>().unwrap() < b_split[1].parse::<i32>().unwrap() {
        return -1;
    }

    // check patch version
    if a_split[2].parse::<i32>().unwrap() > b_split[2].parse::<i32>().unwrap() {
        return 1;
    } else if a_split[2].parse::<i32>().unwrap() < b_split[2].parse::<i32>().unwrap() {
        return -1;
    }

    0
}