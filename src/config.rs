use std::fs::File;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  mirrors: Vec<String>,
  // timeout: u64,
  // max_retries: u64,
  // retry_delay: u64,
  storage_location: String,
  checksum: bool,
}

impl Config {
  pub fn default() -> Self {
    Config {
      mirrors: vec![
        "http://repo.wombatlinux.org".to_string(),
        "https://afroraydude.com/wl/repo".to_string(),
      ],
      // timeout: 10,
      // max_retries: 3,
      // retry_delay: 1,
      storage_location: "/var/uspm/storage".to_string(),
      checksum: false
    }
  }

  pub fn mirrors(&self) -> &Vec<String> {
    &self.mirrors
  }

  pub fn storage_location(&self) -> &String {
    &self.storage_location
  }

  pub fn set_storage_location(&mut self, storage_location: String) {
    self.storage_location = storage_location;
  }

  pub fn add_mirror(&mut self, mirror: String) {
    self.mirrors.push(mirror);
  }

  pub fn remove_mirror(&mut self, mirror: String) {
    self.mirrors.retain(|m| m != &mirror);
  }

  pub fn save(&self) -> Result<(), std::io::Error> {
    let mut file = File::create("/etc/uspm/config.json")?;
    let serialized = serde_json::to_string_pretty(self)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
  }

  pub fn load() -> Result<Self, std::io::Error> {
    let mut file = File::open("/etc/uspm/config.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = serde_json::from_str(&contents)?;
    Ok(config)
  }

  pub fn to_string(&self) -> String {
    serde_json::to_string_pretty(self).unwrap()
  }
}