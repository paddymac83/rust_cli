use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // return std::io::Result<String> from file
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {  // returns and prints a Vec
        println!(" {line} ");
    }

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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {  // line returned references the contents and is valid as long as contents is
    let mut results = Vec::new();
    for line in contents.lines() {  // returns an iterator
            if line.contains(query) {
                results.push(line);
            }
    }
    results
}

// add tests mod in lib.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}