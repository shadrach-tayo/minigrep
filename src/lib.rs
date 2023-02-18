//! # Minigrep package
//! This is a mini version of the popular linux command line tool ``grep``
//!
//! Have fun 🎉
//!

use std::{env, error::Error, fs};

/// Takes an arg of type Config and runs the search function based on the config values
///
/// # Example
/// ```
/// let config = minigrep::Config { query: "you".to_string(), ignore_case: true, file_path: "./poems.txt".to_string() };
/// let result = minigrep::run(config);
/// ```

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("Ignore case {:?}", config.ignore_case);
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    println!("[search_Results]: {:?}", results);

    println!("\n");
    for line in results {
        println!("{line}\n");
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("Not enough arguments");
        // }
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get any query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get any file path"),
        };

        let arg_ignore_case = match args.next() {
            Some(_) => true,
            None => env::var("IGNORE_CASE").is_ok(),
        };

        // let arg_ignore_case = if args.len() > 3 && args[3].contains("--ignore_case") {
        //   true
        // } else {
        //   env::var("IGNORE_CASE").is_ok()
        // };

        // println!("Arg ignore case: {arg_ignore_case}");

        // let env_ignore_case = env::var("IGNORE_CASE").is_ok();
        // println!("env ignore case: {env_ignore_case}");
        Ok(Self {
            query,
            file_path,
            ignore_case: arg_ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
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

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
