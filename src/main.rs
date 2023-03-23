use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();  // convert itr to collection
    // dbg!(args);   // print args, remove to add refs below as dbg! owns args

    let config = Config::new(&args);  // constructor
    // save refs in variables, [0] is the program name
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // return std::io::Result<String> from file
    let contents = fs::read_to_string(config.file_path).expect("Should have a file to read..");

    println!("With text:\n{contents}")
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments");
        }
        let query = args[1].clone();                  // this is a clone as args is owned in main()
        let file_path = args[2].clone();
    
        Config { query, file_path }
    }
}
