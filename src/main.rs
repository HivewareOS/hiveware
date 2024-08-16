use clap::{Arg, Command};
use std::process::{Command as ProcessCommand, exit};

const VERSION: &str = "1.0";

fn main() {
    let matches = Command::new("hiveware")
        .version(VERSION)
        .author("HivewareOS <hiveware@protonmail.com>")
        .about("Nix package management simplified")
        .subcommand(
            Command::new("install")
                .about("Install a package using nix-env")
                .arg(Arg::new("package")
                    .help("The package to install")
                    .required(true)
                    .index(1)),
        )
        .subcommand(
            Command::new("uninstall")
                .about("Uninstall a package using nix-env")
                .arg(Arg::new("package")
                    .help("The package to uninstall")
                    .required(true)
                    .index(1)),
        )
        .subcommand(
            Command::new("virtual")
                .about("Enter a Nix shell with a package installed using nix-shell -p")
                .arg(Arg::new("package")
                    .help("The package to install in the Nix shell")
                    .required(true)
                    .index(1)),
        )
        .subcommand(
            Command::new("version")
                .about("Display the version of hiveware"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            let package = sub_matches.get_one::<String>("package").unwrap();
            install_package(package);
        }
        Some(("uninstall", sub_matches)) => {
            let package = sub_matches.get_one::<String>("package").unwrap();
            uninstall_package(package);
        }
        Some(("virtual", sub_matches)) => {
            let package = sub_matches.get_one::<String>("package").unwrap();
            enter_nix_shell(package);
        }
        Some(("version", _)) => {
            println!("hiveware version {}", VERSION);
        }
        _ => {
            eprintln!("Invalid command! Try hiveware --help for detailed information");
            exit(1);
        }
    }
}

fn install_package(package_arg: &str) {
    let status = ProcessCommand::new("nix-env")
        .args(&["-iA", &format!("nixos.{}", package_arg)])
        .status()
        .expect("Failed to execute nix-env");

    if !status.success() {
        eprintln!("Error installing package {}: {:?}", package_arg, status.code());
        exit(1);
    }
}

fn uninstall_package(package_arg: &str) {
    let status = ProcessCommand::new("nix-env")
        .args(&["--uninstall", package_arg])
        .status()
        .expect("Failed to execute nix-env");

    if !status.success() {
        eprintln!("Error uninstalling package {}: {:?}", package_arg, status.code());
        exit(1);
    }
}

fn enter_nix_shell(package_arg: &str) {
    let status = ProcessCommand::new("nix-shell")
        .arg("-p")
        .arg(package_arg)
        .status()
        .expect("Failed to execute nix-shell");

    if !status.success() {
        eprintln!("Error entering Nix shell with package {}: {:?}", package_arg, status.code());
        exit(1);
    }
}
