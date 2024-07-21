use clap::{Arg, Command};
use clap::builder::ValueRange;
use dit::arguments;
use dit::arguments::add;
use dit::arguments::rm;

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
                .help("add file or files to stage")
        )
        .arg(
            Arg::new("rm")
                .long("rm")
                .num_args(0..)
                .help("add suppressed file or files to stage")
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
}