use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();  // convert itr to collection
    // dbg!(args);   // print args, remove to add refs below as dbg! owns args

    // save refs in variables, [0] is the program name
    let (query, file_path) = parse_config(&args);
    println!("Searching for {}", query);
    println!("In file {}", file_path);

    // return std::io::Result<String> from file
    let contents = fs::read_to_string(file_path).expect("Should have a file to read..");

    println!("With text:\n{contents}")
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)

}