use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // return std::io::Result<String> from file
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)  // no ; as we want result returned
    };

    for line in results {  // returns and prints a Vec
        println!(" {line} ");
    }

    Ok(()) // test comment

}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {  // 'static means it lifetime throughout function
        args.next();  // as first part is prog name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("didnt get query string"),
        };                  // this is a clone as args is owned in main()
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("didnt get file path"),
        }; 

        let ignore_case = env::var("IGNORE_CASE").is_ok();  // Result Ok is env var exists, false if not
    
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {  // line returned references the contents and is valid as long as contents is
    contents
    .lines()
    .filter(|line| line.contains(query))   // filter lines with query and collect into vector  
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {  // line returned references the contents and is valid as long as contents is
    contents
    .lines()
    .filter(|line| line.contains(query))   // filter lines with query and collect into vector  
    .collect()
}

// add tests mod in lib.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}