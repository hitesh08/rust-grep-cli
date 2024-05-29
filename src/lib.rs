use std::{env, fs};
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file: String, 
    pub is_case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config , &str>{
        if args.len() < 3 {
            return Err("\nOops! too less arguments passed.")
        }
        let query = args[1].clone();
        let file = args[2].clone();
        let is_case_sensitive = env::var("CASE_SENSITIVE").is_ok(); // this is not working correctly need checking.
        println!("CASE_SENSITIVE - {}", is_case_sensitive);
        Ok(Config { query, file, is_case_sensitive })
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file)?;
    println!("\n\n{}\n", content);
    let result = if config.is_case_sensitive {
        case_sensitive(&config.query, &content)
    }
    else {
        case_insensitive(&config.query, &content)
    };
    for lines in result {
        println!("{}", lines);
    }
    Ok(())
}

pub fn case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();
    for line in content.lines(){
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}

pub fn case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();
    for line in content.lines(){
        if line.to_lowercase().contains(&query.to_lowercase()) { // doesnt work if i take away &. check why?
            result.push(line)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_test(){
        let query = "Rust";
        let content = "\
Rust:
fast, productive, safe.
voila";
        assert_eq!(vec!["Rust:"], case_sensitive(query, content));
    }

    #[test]
    fn case_insensitive_test(){
        let query = "RUst";
        let content = "\
Rust:
fast, productive, safe.
voila";
        assert_eq!(vec!["Rust:"], case_insensitive(query, content));
    }
}