use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    Ok(())
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
}
