use std::env;
use std::process;
use minigrep::Config; // bring struct into scope of this bin crate (from lib.rs)

fn main() {
    let args: Vec<String> = env::args().collect();  // convert itr to collection
    // dbg!(args);   // print args, remove to add refs below as dbg! owns args

    let config = Config::build(&args).unwrap_or_else(|err| { // use the Err from Result here in a closure
        println!("Exiting on {err}");  // constructor
        process::exit(1);
    });
    // save refs in variables, [0] is the program name
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {  // if fs::read_to_string returns its ? std error, this is handled (propogated) by main
        println!("Encounterted error: {e}");
        process::exit(1);
    }

}
