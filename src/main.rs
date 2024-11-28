use clap::{Arg, Command};
use colored::Colorize;
use std::path::PathBuf;
use std::fs;
use dit::features;
use dit::features::add;
use dit::features::checkout::checkout;
use dit::features::commit::commit;
use dit::features::create_branch::new_branch;
use dit::features::delete::delete;
use dit::features::merge::merge;
use dit::features::message::message;
use dit::features::revert::revert;
use dit::features::rm;
use dit::features::show::show_commit;
use dit::features::display_message::display_message;
use dit::features::display_message::Color;

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
            Command::new("delete").about("Remove committed files").arg(
                Arg::new("files")
                    .help("files to remove")
                    .index(1)
                    .required(true)
                    .num_args(0..)
                    .value_name("FILE(S)"),
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
            Ok(()) => println!("{}", "dit is initialize".green()),
            Err(e) => {
                if PathBuf::from("./.dit").is_dir() {
                   let _ =  fs::remove_dir_all("./.dit");
                }
                display_message("Error initializing dit repository", Color::RED);
                println!("Error while initializing dit repository: {}", e);
            }         
        };
    }

    // ADD
    if let Some(matches) = matches.subcommand_matches("add") {
        if let Some(elements) = matches.get_many::<String>("files") {
            let elements: Vec<_> = elements.collect();
            match add::add(elements) {
                Ok(()) => (),
                Err(e) => panic!("Error while adding elements to dit : {}", e),
            };
        }
    }

    // RM
    if let Some(matches) = matches.subcommand_matches("rm") {
        if let Some(elements) = matches.get_many::<String>("files") {
            let elements: Vec<_> = elements.collect();
            match rm::rm(elements) {
                Ok(()) => (),
                Err(e) => panic!("Error while removing elements to dit : {}", e),
            }
        }
    }

    // DELETE
    if let Some(matches) = matches.subcommand_matches("delete") {
        if let Some(elements) = matches.get_many::<String>("files") {
            let elements: Vec<_> = elements.collect();
            match delete(elements) {
                Ok(()) => (),
                Err(e) => panic!("Error while deleting elements to dit : {}", e),
            }
        }
    }

    // COMMIT
    if let Some(matches) = matches.subcommand_matches("commit") {
        // MESSAGE
        if let Some(mes) = matches.get_one::<String>("message") {
            match message(mes.parse().unwrap()) {
                Ok(()) => (),
                Err(e) => panic!("Error while writing message: {}", e),
            }
            match commit(true) {
                Ok(()) => (),
                Err(e) => panic!("Error while commiting: {}", e),
            }
        }
        // SHOWCOMMIT
        else if matches.get_flag("show") {
            match show_commit() {
                Ok(()) => (),
                Err(e) => panic!("Error while displaying commit tree: {}", e),
            }
        }
        //REVERT
        else if let Some(hash) = matches.get_one::<String>("revert") {
            match revert(hash.to_string()) {
                Ok(()) => (),
                Err(e) => panic!("Error while reverting to the previous state: {e}"),
            }
            // COMMIT
        } else {
            match commit(false) {
                Ok(()) => (),
                Err(e) => panic!("Error while commiting: {}", e),
            }
        }
    }

    // CREATE BRANCH
    if let Some(matches) = matches.subcommand_matches("branch") {
        if let Some(name) = matches.get_one::<String>("branch") {
            match new_branch(name) {
                Ok(()) => (),
                Err(e) => panic!("Error while creating new branch: {e}"),
            }
        }
    }

    // CHECKOUT
    if let Some(matches) = matches.subcommand_matches("checkout") {
        if let Some(name) = matches.get_one::<String>("branch") {
            match checkout(name) {
                Ok(()) => (),
                Err(e) => panic!("Error while changing branch : {e}"),
            }
        }
    }

    // MERGE
    if let Some(matches) = matches.subcommand_matches("merge") {
        if let Some(name) = matches.get_one::<String>("branch") {
            match merge(name) {
                Ok(()) => (),
                Err(e) => panic!("Error while merging branch: {e}"),
            }
        }
    }
}
