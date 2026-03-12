use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("query: \"{query}\", filename: \"{file_path}\"");

    let content = fs::read_to_string(file_path).expect("should have been able to file content");

    println!("content: \n{content}");
}
