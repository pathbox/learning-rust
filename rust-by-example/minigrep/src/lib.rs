use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::env;

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

pub struct Config {
     query: String,
     filename: String,
     case_sensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

//        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        let case_sensitive = true;
        Ok(Config { query, filename, case_sensitive })
    }
}
