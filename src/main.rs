use clap::{Arg, Command};
use clap::builder::{ValueRange};
use dit::arguments;
use dit::arguments::add;
use dit::arguments::checkout::checkout;
use dit::arguments::commit::commit;
use dit::arguments::create_branch::{ new_branch};
use dit::arguments::delete::delete;
use dit::arguments::merge::merge;
use dit::arguments::rm;
use dit::arguments::message::message;
use dit::arguments::revert::revert;
use dit::arguments::show::show_commit;

fn main() {
    let matches = Command::new("dit")
        .version("0.1.0")
        .author("Schaeffer divino divinoschaeffer@gmail.com")
        .about("Local version control tool: Divino Information Tracker ")
        .arg(
            Arg::new("init")
                .short('i')
                .long("init")
                .num_args(ValueRange::EMPTY)
                .help("initialize dit repository")
        )
        .arg(
            Arg::new("add")
                .short('a')
                .long("add")
                .num_args(0..)
                .value_name("FILE")
                .help("add files or directories to stage")
        )
        .arg(
            Arg::new("rm")
                .long("rm")
                .num_args(0..)
                .value_name("FILE")
                .help("suppressed file or files from stage")
        )
        .arg(
            Arg::new("commit")
                .short('c')
                .long("commit")
                .num_args(0)
                .help("commit all staged elements")
        )
        .arg(
            Arg::new("message")
                .short('m')
                .long("message")
                .num_args(1)
                .value_name("DESCRIPTION")
                .help("commit message")
        )
        .arg(
            Arg::new("delete")
                .short('d')
                .long("delete")
                .value_name("FILE")
                .num_args(0..)
                .help("suppress commited elements")
        )
        .arg(
            Arg::new("commit_tree")
                .long("showcommit")
                .alias("sc")
                .num_args(0)
                .help("show commit tree")
        )
        .arg(
            Arg::new("revert")
                .long("revert")
                .short('r')
                .num_args(1)
                .value_name("COMMIT ID")
                .help("revert files to their state at a specified commit.")
        )
        .arg(
            Arg::new("new_branch")
                .long("newbanch")
                .alias("nb")
                .num_args(1)
                .value_name("NAME")
                .help("create a new branch and switch to the branch")
        )
        .arg(
            Arg::new("checkout")
                .long("checkout")
                .num_args(1)
                .value_name("NAME")
                .help("change the current branch")
        )
        .arg(
            Arg::new("merge")
                .long("merge")
                .num_args(1)
                .value_name("NAME")
                .help("merge current branch with the specified branch")
        )
        .get_matches();

    // INIT
    if matches.get_flag("init") {
        match arguments::init::init_repository(){
            Ok(()) => println!("dit is initialized"),
            Err(e) => panic!("Error while initializing dit repository: {}",e),
        };
    }

    // ADD
    if let Some(elements) = matches.get_many::<String>("add") {
        let elements: Vec<_> = elements.collect();
        match add::add(elements) {
            Ok(()) => (),
            Err(e) => panic!("Error while adding elements to dit : {}",e),
        };
    }

    // RM
    if let Some(elements) = matches.get_many::<String>("rm") {
        let elements: Vec<_> = elements.collect();
        match rm::rm(elements) {
            Ok(()) => (),
            Err(e) => panic!("Error while removing elements to dit : {}", e)
        }
    }

    // DELETE
    if let Some(elements) = matches.get_many::<String>("delete") {
        let elements: Vec<_> = elements.collect();
        match delete (elements) {
            Ok(()) => (),
            Err(e) => panic!("Error while deleting elements to dit : {}", e)
        }
    }
    
    // COMMIT
    if matches.get_flag("commit") {
        match commit() { 
            Ok(()) => (),
            Err(e) => panic!("Error while commiting: {}",e)
        }
    }
    
    // MESSAGE
    if let Some(mes) = matches.get_one::<String>("message"){
        match message(mes.parse().unwrap()) { 
            Ok(()) => (),
            Err(e) => panic!("Error while writing message: {}", e)
        }
    }
    
    // SHOWCOMMIT
    if matches.get_flag("commit_tree"){
        match show_commit() {
            Ok(()) => (),
            Err(e) => panic!("Error while displaying commit tree: {}", e)
        }
    }
    
    // REVERT
    if let Some(hash) = matches.get_one::<String>("revert") {
        match revert(hash.to_string()) { 
            Ok(()) => (),
            Err(e) => panic!("Error while reverting to the previous state: {e}")
        }
    }
    
    // CREATE BRANCH
    if let Some(name) = matches.get_one::<String>("new_branch"){
        match new_branch(name) {
            Ok(()) => (),
            Err(e) => panic!("Error while creating new branch: {e}")
        }
    }
    
    // CHECKOUT
    if let Some(name) = matches.get_one::<String>("checkout"){
        match checkout(name) {
            Ok(()) => (),
            Err(e) => panic!("Error while changing branch : {e}")
        }
    }
    
    // MERGE
    if let Some(name) = matches.get_one::<String>("merge"){
        match merge(name) {
            Ok(()) => (),
            Err(e) => panic!("Error while merging branch: {e}")
        }
    }
}