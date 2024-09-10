use clap::{Arg, Command};
use clap::builder::ValueRange;
use dit::arguments;
use dit::arguments::add;
use dit::arguments::commit::commit;
use dit::arguments::delete::delete;
use dit::arguments::rm;
use dit::arguments::message::message;
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
                .num_args(0)
                .help("chow commit tree")
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
}