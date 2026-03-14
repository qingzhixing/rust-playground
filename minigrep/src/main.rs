use std::{env, error::Error, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Usage: minigrep <query> <file_path>");
        eprintln!("Error: {err}");
        std::process::exit(1);
    });

    println!(
        "Searching for {query} in file {file_path}",
        query = config.query,
        file_path = config.file_path
    );

    if let Err(e) = run(config) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;

    println!("content: \n{content}");

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
