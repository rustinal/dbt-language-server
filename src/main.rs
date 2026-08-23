mod definition;
mod parser;
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    // args[0] is the program name, args[1] is the file path
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_path = &args[1];
    match fs::read_to_string(file_path) {
        Ok(contents) => println!("File content:\n{}", contents),
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}
