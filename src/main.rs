use std::env;
use install::install_package;

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
            println!("remove");
        },
        "update" => {
            // update the packages
            println!("update");
        },
        "upgrade" => {
            // upgrade the packages
            println!("upgrade");
        },
        "search" => {
            // search for the packages
            println!("search");
        },
        "list" => {
            // list the packages
            println!("list");
        },
        "info" => {
            // info about the packages
            println!("info");
        },
        "config" => {
            // config the packages
            println!("config");
        },
        _ => {
            // print help
            print_help()
        }
    }
}

fn print_help() {
    println!("Usage: uspm-rust <command> <package 1> <package 2> <package 3> ...");
    println!("Commands:");
    println!("  install <package 1> <package 2> <package 3> ...");
    println!("  remove <package 1> <package 2> <package 3> ...");
    println!("  update");
    println!("  upgrade");
}
