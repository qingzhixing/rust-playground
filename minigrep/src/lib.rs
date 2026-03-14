pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn search(config: &Config, content: &str) -> Vec<String> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(&config.query) {
            results.push(line.to_string());
        }
    }
    results
}
