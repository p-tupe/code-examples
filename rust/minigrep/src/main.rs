use std::{env, error::Error, fs, process};

use minigrep::search;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }

    Ok(())
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    search(
        &config.query,
        &fs::read_to_string(config.path)?,
        config.ignore_case,
    )
    .iter()
    .for_each(|l| println!("{l}"));

    Ok(())
}

struct Config {
    query: String,
    path: String,
    ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // Ignore program file

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            path,
            ignore_case,
        })
    }
}
