use std::env;
use install::install_package;
use crate::config::Config;
use crate::install::uninstall_package;
use crate::repo::{check_repo_for_package, download_repo_file};

mod install;
mod dephandle;
mod config;
mod package;
mod repo;

/**
This is a rust version of the entire USPM project. I am using this to learn Rust.
Use as uspm-rust <command> <package 1> <package 2> <package 3> ...
 */
#[tokio::main]
async fn main() {
    // command line arguments
    let args: Vec<String> = env::args().collect();

    // if there are no args, print_help()
    if args.len() == 1 {
        print_help();
        return;
    }

    // match the first argument to a command
    match args[1].as_str() {
        "install" => {
            // install the packages
            // for each package, install
            for package in args[2..].iter() {
                install_package(package.to_string()).expect("Could not install package");
            }
        },
        "remove" => {
            // remove the packages
            for package in args[2..].iter() {
                uninstall_package(package.to_string()).expect("Could not install package");
            }
        },
        "upgrade" => {
            // upgrade the packages
            for package in args[2..].iter() {
                // this basically just tries to install the package again
                // it won't install if the current version is greater than or equal to the version in the repo
                install_package(package.to_string()).expect("Could not install package");
            }
        },
        "search" => {
            // search for the packages
            for package in args[2..].iter() {
                search(package.to_string()).await;
            }
        },
        "list" => {
            // list the packages
            list_packages();
        },
        "config" => {
            config();
        },
        "help" => {
            // print help
            print_help();
        },
        "version" => {
            // print version
            println!("uspm-rust 0.0.1");
        },
        _ => {
            // print help
            print_help()
        }
    }
}

async fn search(package: String) {
    // load the config file
    let config: Config;
    let config_result = Config::load();
    if config_result.is_err() {
        config = Config::default();
        config.save().expect("Could not save default config")
    } else {
        config = config_result.unwrap();
    }

    for mirror in config.mirrors() {
        let repo_result = download_repo_file(mirror).await;
        if repo_result.is_err() {
            println!("Could not download repo file from {}", mirror);
            continue;
        } else {
            let repo = repo_result.unwrap();
            if check_repo_for_package(repo, &package) {
                println!("Found package {} in {}", package, mirror);
            }
        }
    }
}

fn print_help() {
    println!("Usage: uspm-rust <command> [<package 1> <package 2> <package 3> ...]");
    println!("Commands:");
    println!("  install <package 1> [<package 2> <package 3> ...]");
    println!("  remove <package 1> [<package 2> <package 3> ...]");
    println!("  upgrade");
    println!("  search");
    println!("  list");
    println!("  config");
    println!("  help");
    println!("  version");
}

fn config() {
    let config: Config;
    let config_result = Config::load();

    // If the config file doesn't exist, create it and use the default config.
    if config_result.is_err() {
        config = Config::default();
        config.save().expect("Could not save default config")
    } else {
        config = config_result.unwrap();
    }

    println!("Config:\n{}", config.to_string());
}

fn list_packages() {
    // list all packages
    let mut package_file = package::Packages::new();
    if package_file.load().is_err() {
        println!("Packages file could not be loaded! Making new packages file");
        package_file.save().expect("Could not save packages file, exiting");
        println!("New packages file, no packages installed");
        return;
    } else {
        package_file.load().unwrap();
    }
    for package in package_file.get_packages() {
        println!("{} {}", package.name, package.version);
    }
}


