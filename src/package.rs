use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use md5;

use serde::__private::de;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageFile {
    pub name: String,
    pub(crate) version: String,
    pub dependencies: HashMap<String, String>,
    pub checksum: String,
}


impl Clone for PackageFile {
    fn clone(&self) -> Self {
        PackageFile {
            name: self.name.clone(),
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

    pub fn get_packages(&self) -> Vec<PackageFile> {
        self.packages.values().cloned().collect()
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

    pub fn check_hash(path: String, hash: String) -> bool {
        let bytes = std::fs::read(path).unwrap();
        let digest = md5::compute(bytes);
        let hex = format!("{:x}", digest);
        hex == hash
    }

    pub fn default() -> Self {
        let mut dependencies = HashMap::new();
        dependencies.insert("uspm".to_string(), "1.0.0".to_string());
        PackageFile {
            name: "test".to_string(),
            version: "0.0.0".to_string(),
            dependencies: dependencies,
            checksum: "".to_string(),
        }
    }

    pub fn check(&self) -> bool {
        // check if name is empty
        if self.name.is_empty() {
            return false;
        }
        // check if version is empty
        if self.version.is_empty() {
            return false;
        }
        // check if any of the values in dependencies are empty
        for (key, value) in self.dependencies.iter() {
            if key.is_empty() || value.is_empty() {
                return false;
            }

            // check if value doesn't match valid version format (#.#.#) using regex
            let re = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
            if !re.is_match(value) {
                return false;
            }
        }
        // check if checksum is empty
        if !self.checksum.is_empty() && self.checksum.as_str() != "leave_blank_in_package_file" {
            return false;
        }
        true
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