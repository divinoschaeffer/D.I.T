use clap::{Arg, Command};
use clap::builder::ValueRange;
use dit::arguments;
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
        .get_matches();
    
    if matches.get_flag("init") {
        match arguments::init::init_repository(){
            Ok(()) => (),
            Err(e) => panic!("Error while initializing dit repository: {e}"),
        };
    }
}