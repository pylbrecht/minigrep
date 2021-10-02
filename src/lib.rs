use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    Ok(())
}

fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_constructor() {
        let args = vec![
            "minigrep".to_string(),
            "some query".to_string(),
            "some/file.txt".to_string(),
        ];
        let config = Config::new(&args).unwrap();
        assert_eq!(&config.query, "some query");
        assert_eq!(&config.filename, "some/file.txt");
    }

    #[test]
    fn not_enough_args() {
        let args = Vec::new();
        let config = Config::new(&args);
        assert_eq!(config.unwrap_err(), "not enough arguments");
    }

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
