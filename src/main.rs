use std::env;
use std::fs::{self}; // Added to use command-line arguments

fn main() {
    let args: Vec<String> = env::args().collect(); // Collect command-line arguments
    let path = if args.len() > 1 {
        // Check if a path is provided
        &args[1] // Use the first argument as the path
    } else {
        "C:/" // Default path if none is provided
    };

    let entries = fs::read_dir(path).unwrap(); // Use the provided path
                                               // modify code for input path in terminal after run code
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            println!("Directory: {}", path.display());
        } else {
            println!("{}", path.display());
        }
    }
}
