use std::{env, error::Error, fs, process};

use minigrep::search;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();
    let config = Config::build(&args).unwrap_or_else(|err| {
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
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Invalid arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            path: args[2].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}
