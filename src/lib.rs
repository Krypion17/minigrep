use std::{error::Error, fs, env, process};

pub const NORMAL: &str = "\x1b[0m";
pub const RED: &str = "\x1b[1;31m";

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => {
                if !(arg.contains("help") || arg.contains("-h")) {
                    arg
                } else {
                    help();
                    process::exit(0);
                }
            },
            None => { 
                help(); 
                process::exit(0);
            },
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = is_case_sensitive(args);

        Ok(Config{ query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive { 
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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .map(|line| {
            let mut line = String::from(line);
            let idx = line.find(&query).unwrap_or_else(||{
                eprint!("Not found");
                process::exit(1);
            });
            line.insert_str(idx, RED);
            line.insert_str(idx + query.len() + 7, NORMAL);
            line
        })
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .map(|line| {
            let mut line = String::from(line);
            let idx = line.to_lowercase().find(&query.to_lowercase()).unwrap_or_else(||{
                eprint!("Not found");
                process::exit(1);
            });
            line.insert_str(idx, RED);
            line.insert_str(idx + query.len() + 7, NORMAL);
            line
        })
        .collect()

}

pub fn help() {
    println!("Minigrep
Usage: minigrep [QUERY] [FILE]
Search for QUERY in FILE.
Flags:
  -h, --help             Display this message
      --case_sensitive   Search with case sensitivity (default)
      --case_insensitive Search without case sensitivity");
}

fn is_case_sensitive(mut args: env::Args) -> bool {
    match args.next() {
        Some(arg) => {
            if arg == "--case_sensitive" {
                true
            } else if arg == "--case_insensitive" {
                false
            } else {
                false
            }
        },
        None => env::var("CASE_INSENSITIVE").is_err(),
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
