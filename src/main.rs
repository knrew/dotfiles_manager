use core::panic;
use std::{env, path::PathBuf};

use chrono::Local;
use clap::{arg, command, value_parser, Command};

use dotfiles_manager::install::install;

fn main() {
    let command = command!()
        .subcommand(
            Command::new("install")
                .arg(
                    arg!([dotfiles_dir] "dotfiles [default: $HOME/dotfiles/]")
                        .value_parser(value_parser!(PathBuf))
                        .required(false),
                )
                .arg(
                    arg!([install_dir] "install directory [default: $HOME/]")
                        .value_parser(value_parser!(PathBuf))
                        .required(false),
                )
                .arg(
                    arg!([backup_dir] "backup directory [default: $HOME/.backup_dotfiles/]")
                        .value_parser(value_parser!(PathBuf))
                        .required(false),
                ),
        )
        .subcommand(Command::new("backup"));

    let mut cmd = command.clone();

    match command.get_matches().subcommand() {
        Some(("install", args)) => {
            println!("installing dotfiles...");

            let dotfiles_dir = if let Some(path) = args.get_one::<PathBuf>("dotfiles_dir") {
                path.clone()
            } else {
                let home_dir = env::var("HOME").expect("HOME not found");
                PathBuf::from(home_dir).join("dotfiles")
            };

            let install_dir = if let Some(path) = args.get_one::<PathBuf>("install_dir") {
                path.clone()
            } else {
                let home_dir = env::var("HOME").expect("HOME not found");
                PathBuf::from(home_dir).join("install")
            };

            let today = Local::now().format("%Y%m%d_%H%M").to_string();
            let backup_dir = if let Some(path) = args.get_one::<PathBuf>("backup_dir") {
                path.join(today)
            } else {
                let home_dir = env::var("HOME").expect("HOME not found");
                PathBuf::from(home_dir).join("backup").join(today)
            };

            println!("dotfiles directory: {:?}", dotfiles_dir);
            println!("install directory: {:?}", install_dir);
            println!("backup directory: {:?}", backup_dir);

            install(dotfiles_dir, install_dir, backup_dir);
        }
        Some(("backup", _)) => {
            unimplemented!("backup");
        }
        _ => {
            cmd.print_help().unwrap();
            panic!();
        }
    }
}
