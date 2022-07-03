//! # Minigrep
//!
//! `minigrep` is my version of `grep`. It uses rust to search for strings in files! 🐙
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        Ok(Config {
            query: match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string"),
            },
            filename: match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a filename"),
            },
            case_sensitive: match args.next() {
                Some(arg) => arg != "--insensitive",
                None => env::var("CASE_INSENSITIVE").is_err(),
            },
        })
    }
}

/// # Run is the shiz
///
/// Pass in a config struct, and it will do the magic.
/// Boom! Done. 😀
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for el in results {
        println!("{}", el);
    }

    Ok(())
}

/// Returns a list of lines that contain a given query
///
/// # Examples
///
/// ```
/// use minigrep::search;
///
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Duct tape.";
///
/// assert_eq!(vec!["safe, fast, productive."], search(query, contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|x| x.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|x| x.to_lowercase().contains(&query.to_lowercase()))
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
            search_case_insensitive(query, contents),
        );
    }
}
