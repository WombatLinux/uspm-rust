use std::collections::HashMap;
use crate::package;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
  pub packages: HashMap<String, package::PackageFile>
}

impl Clone for Repo {
  fn clone(&self) -> Self {
    Repo {
      packages: self.packages.clone()
    }
  }
}

pub async fn download_repo_file(mirror: &String) -> Result<Repo, reqwest::Error>{
  let repo_url = mirror.to_string() + "/repo.json";
  let response = reqwest::get(repo_url)
      .await?;
  let repo_file = response.json::<Repo>().await?;
  Ok(repo_file)
}

pub fn check_repo_for_package(repo_file: Repo, package: &String) -> bool {
  return repo_file.packages.contains_key(&*package);
}

impl Repo {
    pub fn get_package(&self, name: String) -> Option<&package::PackageFile> {
        self.packages.get(&name)
    }

    pub fn default() -> Self {
      let package_file = package::PackageFile::default();
      let mut packages = HashMap::new();
      packages.insert("test".to_string(), package_file);
      Repo {
        packages: packages
      }
    }
}