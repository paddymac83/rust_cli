use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // return std::io::Result<String> from file
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())

}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {  // 'static means it lifetime throughout function
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();                  // this is a clone as args is owned in main()
        let file_path = args[2].clone();
    
        Ok(Config { query, file_path })
    }
}