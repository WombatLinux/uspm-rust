use std::collections::HashMap;
use crate::package;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
  packages: HashMap<String, package::PackageFile>
}

pub async fn download_repo_file(mirror: &String) -> Result<Repo, reqwest::Error>{
  let response = reqwest::get(mirror)
      .await?;
  let repo_file: Repo = response.json().await?;
  Ok(repo_file)
}

pub fn check_repo_for_package(repo_file: Repo, package: &String) -> bool {
  return repo_file.packages.contains_key(&*package);
}