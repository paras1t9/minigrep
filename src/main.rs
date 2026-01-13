use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("Searching for {}", config.query);

    println!("In the file {}", config.file_path);
    let contents = fs::read_to_string("poem.txt").expect("Should have been able to read the file");
    println!("The content :\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    let config = Config { query, file_path };
    config
}
