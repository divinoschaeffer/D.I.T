use std::{fs, process};
use std::path::PathBuf;

use clap::{Arg, Command};

use dit::features;
use dit::features::add;
use dit::features::checkout::checkout;
use dit::features::commit::commit;
use dit::features::create_branch::new_branch;
use dit::features::display_message::Color;
use dit::features::display_message::display_message;
use dit::features::merge::merge;
use dit::features::message::message;
use dit::features::revert::revert;
use dit::features::rm;
use dit::features::show::show_commit;

fn main() {
    let matches = Command::new("cli")
        .version("0.1.0")
        .author("Schaeffer divino divinoschaeffer@gmail.com")
        .about("Local version control tool: Divino Information Tracker ")
        .subcommands([
            Command::new("init").about("Initialize dit repository"),
            Command::new("add").about("Index file(s)").arg(
                Arg::new("files")
                    .help("files to add")
                    .index(1)
                    .required(true)
                    .num_args(0..)
                    .value_name("FILE(S)"),
            ),
            Command::new("rm").about("Remove index file(s)").arg(
                Arg::new("files")
                    .help("Files to remove")
                    .index(1)
                    .required(true)
                    .num_args(0..)
                    .value_name("FILE(S)"),
            ),
            Command::new("commit")
                .about("Commit")
                .arg(
                    Arg::new("message")
                        .short('m')
                        .required(false)
                        .num_args(1)
                        .value_name("MESSAGE")
                        .help("Commit message"),
                )
                .arg(
                    Arg::new("show")
                        .help("show commit tree")
                        .required(false)
                        .short('s')
                        .num_args(0),
                )
                .arg(
                    Arg::new("revert")
                        .short('r')
                        .required(false)
                        .num_args(1)
                        .value_name("COMMIT ID")
                        .help("Revert files to their state at a specified commit."),
                ),
            Command::new("branch").about("Branch").arg(
                Arg::new("branch")
                    .num_args(1)
                    .index(1)
                    .required(true)
                    .value_name("NAME")
                    .help("Create a new branch and switch to the branch"),
            ),
            Command::new("checkout").about("Checkout").arg(
                Arg::new("branch")
                    .num_args(1)
                    .index(1)
                    .required(true)
                    .value_name("NAME")
                    .help("Switch to the branch"),
            ),
            Command::new("merge").about("Merge").arg(
                Arg::new("branch")
                    .num_args(1)
                    .index(1)
                    .required(true)
                    .value_name("NAME")
                    .help("Merge branch with the current branch"),
            ),
        ])
        .get_matches();

    // INIT
    if let Some(_) = matches.subcommand_matches("init") {
        match features::init::init_repository() {
            Ok(()) => display_message("dit initialized.", Color::GREEN),
            Err(e) => {
                if PathBuf::from("./.dit").is_dir() {
                    let _ = fs::remove_dir_all("./.dit");
                }
                display_message(format!("Error initializing dit repository: {}.", e).as_str(), Color::RED);
                process::exit(1);
            }
        };
    }

    // ADD
    if let Some(matches) = matches.subcommand_matches("add") {
        if let Some(elements) = matches.get_many::<String>("files") {
            let elements: Vec<_> = elements.collect();
            match add::add(elements) {
                Ok(()) => display_message("Files added.", Color::GREEN),
                Err(e) => {
                    display_message(format!("Error adding elements to dit : {}.", e).as_str(), Color::RED);
                    process::exit(1);
                }
            };
        }
    }

    // RM
    if let Some(matches) = matches.subcommand_matches("rm") {
        if let Some(elements) = matches.get_many::<String>("files") {
            let elements: Vec<_> = elements.collect();
            match rm::rm(elements) {
                Ok(()) => (),
                Err(e) => {
                    display_message(format!("Error removing elements to dit : {}", e).as_str(), Color::RED);
                    process::exit(1);
                }
            }
        }
    }

    // COMMIT
    if let Some(matches) = matches.subcommand_matches("commit") {
        // MESSAGE
        if let Some(mes) = matches.get_one::<String>("message") {
            match message(mes.parse().unwrap()) {
                Ok(()) => (),
                Err(e) => {
                    display_message(format!("Error writing message: {}.", e).as_str(), Color::RED);
                    process::exit(1);
                }
            }
            match commit(true) {
                Ok(()) => display_message("Commit created.", Color::GREEN),
                Err(e) => {
                    display_message(format!("Error commiting elements: {}.", e).as_str(), Color::RED);
                    process::exit(1);
                }
            }
        }
        // SHOWCOMMIT
        else if matches.get_flag("show") {
            match show_commit() {
                Ok(()) => (),
                Err(e) => {
                    display_message(format!("Error displaying commit tree: {}", e).as_str(), Color::RED);
                    process::exit(1);
                }
            }
        }
        //REVERT
        else if let Some(hash) = matches.get_one::<String>("revert") {
            match revert(hash.to_string()) {
                Ok(()) => (),
                Err(e) => {
                    display_message(format!("Error reverting to the previous state: {e}").as_str(), Color::RED);
                    process::exit(1);
                }
            }
            // COMMIT
        } else {
            match commit(false) {
                Ok(()) => (),
                Err(e) => {
                    display_message(format!("Error while commiting: {}", e).as_str(), Color::RED);
                    process::exit(1);
                }
            }
        }
    }

    // CREATE BRANCH
    if let Some(matches) = matches.subcommand_matches("branch") {
        if let Some(name) = matches.get_one::<String>("branch") {
            match new_branch(name) {
                Ok(()) => (),
                Err(e) => {
                    display_message(format!("Error while creating new branch: {e}").as_str(), Color::RED);
                    process::exit(1);
                }
            }
        }
    }

    // CHECKOUT
    if let Some(matches) = matches.subcommand_matches("checkout") {
        if let Some(name) = matches.get_one::<String>("branch") {
            match checkout(name) {
                Ok(()) => (),
                Err(e) => {
                    display_message(format!("Error while changing branch : {e}").as_str(), Color::RED);
                    process::exit(1);
                }
            }
        }
    }

    // MERGE
    if let Some(matches) = matches.subcommand_matches("merge") {
        if let Some(name) = matches.get_one::<String>("branch") {
            match merge(name) {
                Ok(()) => (),
                Err(e) => {
                    display_message(format!("Error while merging branch: {e}").as_str(), Color::RED);
                    process::exit(1);
                }
            }
        }
    }
    process::exit(0);
}
