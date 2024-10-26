use std::fs::{self};
use std::io::{self, Write};

fn create_file(filename: &str) {
    match fs::File::create(filename) {
        Ok(_) => println!("File '{}' created successfully!", filename),
        Err(e) => println!("Error creating file: {}", e),
    }
}

fn read_directory(path: &str) {
    match fs::read_dir(path) {
        Ok(entries) => {
            println!("\nListing contents of directory: {}", path);
            println!("----------------------------------------");
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            println!("📁 Directory: {}", path.display());
                        } else {
                            println!("📄 File: {}", path.display());
                        }
                    }
                    Err(e) => println!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => println!("Error reading directory: {}", e),
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    loop {
        println!("\n=== File Management System ===");
        println!("1. Create File");
        println!("2. Read Directory");
        println!("3. Exit");
        println!("=========================");

        let choice = get_user_input("Enter your choice (1-3): ");

        match choice.as_str() {
            "1" => {
                let filename = get_user_input("Enter filename to create: ");
                create_file(&filename);
            }
            "2" => {
                let path = get_user_input("Enter directory path to read: ");
                read_directory(&path);
            }
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice! Please select 1, 2, or 3."),
        }
    }
}
