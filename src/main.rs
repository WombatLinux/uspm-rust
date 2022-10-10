use std::env;
use install::install_package;
use crate::install::uninstall_package;

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
            println!("search");
        },
        "list" => {
            // list the packages
            list_packages();
        },
        "config" => {
            // config the packages
            println!("config");
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

fn list_packages() {
    // list all packages
    let mut package_file = package::Packages::new();
    package_file.load().expect("Could not load packages");
    for package in package_file.get_packages() {
        println!("{} {}", package.name, package.version);
    }
}


