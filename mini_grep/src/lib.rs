use std::{fs, env};
use std::error::Error;

pub struct Config {
    pub filename: String,
    pub query: String,
    pub case_sensitivity: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("Invalid number of arguements supplied");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitivity = env::var("CASE_SENSITIVITY").is_err();

        Ok(Config { query, filename, case_sensitivity })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitivity {
        search(&config.query, &contents)
    }else {
        search_insensitive(&config.query, &contents)
    };

    for line in results{
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    let query = &query.to_lowercase();

    for line in contents.lines(){
        if line.to_lowercase().contains(query){
            results.push(line);
        }

    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "/Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    
    #[test]
    fn case_insensitive() {
        let query = "DUcT";
        let contents = "/Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_insensitive(query, contents));
    }
}