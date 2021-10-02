use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    let contents = fs::read_to_string(config.filename).expect("error reading the file");
}

fn parse_config(args: &[String]) -> Config {
    Config {
        query: args[1].clone(),
        filename: args[2].clone(),
    }
}

struct Config {
    query: String,
    filename: String,
}
