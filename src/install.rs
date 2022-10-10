use std::fmt::Error;
use std::fs::File;
use std::io::{Bytes, Write};
use std::process::Command;
use crate::config::Config;
use crate::dephandle::check_dependency;
use crate::package::{PackageFile, Packages};
use crate::repo::{check_repo_for_package, download_repo_file};

// TODO: Read: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types.html
// see how to process multiple errors

async fn get_package_from_mirror(url: String) -> Result<Vec<u8>, reqwest::Error> {
    let response = reqwest::get(url.as_str()).await?;
    let package_file = response.bytes().await?.to_vec();
    Ok(package_file)
}

#[tokio::main]
pub async fn download_package(package: String) -> Result<bool, std::io::Error> {
    let mut config: Config;
    let mut url: String = "".to_string();
    let config_result = Config::load();

    // If the config file doesn't exist, create it and use the default config.
    if config_result.is_err() {
        config = Config::default();
        config.save().expect("Could not save default config")
    } else {
        config = config_result.unwrap();
    }

    // attempt to download the package from the first mirror in the config file
    // that has the package
    for mirror in config.mirrors() {
        let repo_result = download_repo_file(mirror).await;
        if repo_result.is_err() {
            continue;
        } else {
            let repo = repo_result.unwrap();
            if check_repo_for_package(repo, &package) {
                url = mirror.to_string();
            }
        }
    }

    // if the package was not found in any mirror, return false
    if url.is_empty() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Package not found"));
    }

    // download package to storage directory
    // add package file to url

    url = url + "/" + &package + ".uspm";

    let mut file_path = config.storage_location().to_string() + "/" + &package + ".uspm";
    let mut file = File::create(file_path)?;
    let package_result = get_package_from_mirror(url).await;
    if package_result.is_err() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Package not found"));
    }

    let package = package_result.unwrap();

    // convert vector to &[u8]
    let package_slice = &package[..];

    file.write_all(package_slice)?;

    Ok(true)
}

pub fn install_package(package: String) -> Result<bool, std::io::Error> {
    let mut config: Config;
    let config_result = Config::load();

    // If the config file doesn't exist, create it and use the default config.
    if config_result.is_err() {
        config = Config::default();
        config.save().expect("Could not save default config")
    } else {
        config = config_result.unwrap();
    }

    // first see if the package file exists in the storage directory
    let file_path = config.storage_location().to_string() + "/" + &package + ".uspm";
    // if it doesn't, download it
    if !std::path::Path::new(&file_path).exists() {
        let download_result = download_package(package.clone());
        if download_result.is_err() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Package not found"));
        }
    }

    // now we need to extract the package file and check its package.json file
    // to see if it has any dependencies
    // if it does, we need to install those dependencies
    // then we need to run the install script

    // extract package file with tar
    Command::new("tar")
        .arg("-xvf")
        .arg(&file_path)
        .current_dir(&config.storage_location())
        .output()
        .expect("failed to execute process");
    // check package.json for dependencies by going to the package directory
    // and reading the package.json file
    let package_json = config.storage_location().to_string() + "/" + &package + "/package.json";
    let package_load = PackageFile::load(package_json);

    if package_load.is_err() {
        // crash
        panic!("Package files issue! Please delete offending files and try again")
    }

    let p_file = package_load.unwrap();

    // install dependencies
    for dependency in p_file.clone().dependencies {
        // check dependency
        if check_dependency(dependency.0.clone(), dependency.1) {
            continue;
        }

        // install dependency
        let install_result = install_package(dependency.0.clone());

        if install_result.is_err() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Package not found"));
        }
    }

    // run install script
    let install_script = config.storage_location().to_string() + "/" + &package + "/install.sh";
    Command::new("sh")
        .arg(&install_script)
        .current_dir(&config.storage_location())
        .output()
        .expect("failed to execute process");

    // save package to installed packages list
    let mut packages = Packages::new();

    if packages.load().is_err() {
        println!("Packages file could not be loaded! Making new packages file");
    } else {
        packages.load().unwrap();
    }

    packages.add_package(package.clone(), p_file);

    Ok(true)
}