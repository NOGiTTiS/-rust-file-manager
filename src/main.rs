// src/main.rs/
//from src/main.rs/ create code for use cd command
use std::fs::{self, DirEntry};
use std::io::{self, Write};
use std::path::Path;

struct FileManager {
    current_dir: String,
}

impl FileManager {
    fn new(starting_dir: &str) -> FileManager {
        FileManager {
            current_dir: starting_dir.to_string(),
        }
    }

    fn list_files(&self) {
        let paths = std::fs::read_dir(&self.current_dir).unwrap();
        for path in paths {
            println!("{}", path.unwrap().path().display());
        }
    }

    fn cd(&mut self, new_dir: &str) {
        let new_path = if new_dir.starts_with("/") {
            new_dir.to_string()
        } else {
            let mut current_path = Path::new(&self.current_dir).to_path_buf();
            current_path.push(new_dir);
            current_path.to_str().unwrap().to_string()
        };
        if Path::new(&new_path).exists() {
            self.current_dir = new_path;
        } else {
            println!("Error: Directory not found.");
        }
    }
    // You can add other methods to FileManager here
}

fn main() {
    let mut file_manager = FileManager::new(".");
    file_manager.cd("src");
    file_manager.list_files();
    
    loop {
        println!("Current directory: {}", file_manager.current_dir);
        println!("Commands: list, cd, create_file, create_dir, delete_file, delete_dir, quit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        match input {
            "list" => file_manager.list_files(),
            "quit" => break,
            _ => println!("Invalid command"),
        }
    }
}
