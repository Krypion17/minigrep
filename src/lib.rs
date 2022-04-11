use std::{error::Error, fs, env, process};

pub const NORMAL: &str = "\x1b[0m";
pub const RED: &str = "\x1b[1;31m";

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = is_case_sensitive(args);

        Ok(Config{ query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive{ 
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            let mut line = String::from(line);
            let idx = line.find(&query).unwrap_or_else(||{
                eprint!("Not found");
                process::exit(1);
            });
            line.insert_str(idx, RED);
            line.insert_str(idx + query.len() + 7, NORMAL);
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            let mut line = String::from(line);
            let idx = line.to_lowercase().find(&query).unwrap_or_else(||{
                eprint!("Not found");
                process::exit(1);
            });
            line.insert_str(idx, RED);
            line.insert_str(idx + query.len() + 7, NORMAL);
            results.push(line);
        }
    }

    results

}

fn is_case_sensitive(args: &[String]) -> bool {
    if args.len() < 4 {
        return env::var("CASE_INSENSITIVE").is_err();
    }

    if args[3] == "--case_sensitive" {
        true
    } else if args[3] == "--case_insensitive" {
        false
    } else {
        false
    }
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
Pick three.
Duct tape.";
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
