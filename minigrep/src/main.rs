use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(msg) => {
            eprintln!("Usage: minigrep <query> <file_path>");
            eprintln!("Error: {msg}");
            std::process::exit(1);
        }
    };

    let content =
        fs::read_to_string(&config.file_path).expect("should have been able to file content");

    println!("content: \n{content}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
