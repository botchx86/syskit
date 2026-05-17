use std::env;
use clap::{Arg, command, ArgGroup};
use std::path::PathBuf;
use multitool::find_file;

fn main() {

    dashboard();

    let _matches = command!()
    .about("This is a multitool written in Rust to centralise a lot of primary tools into a single tool.")

    .arg(
        Arg::new("find")
            .short('f')
            .long("find")
            .help("Find a file")
            .num_args(1)
            .value_name("FILENAME")
    )
    
    .arg(
        Arg::new("read")
            .short('r')
            .long("read")
            .help("Read a file")
            .num_args(1)
            .value_name("FILEPATH")
    )
    .get_matches();

    if let Some(file_to_find) = matches.get_string("find") {
        println!("Searching for {}", file_to_find);

        let current_dir = Path::new(".");
        find_file(&file_to_find, current_dir);
    }


}

fn dashboard() {
    println!("Welcome to the Rust Multitool");
    println!("-----------------------------");
}