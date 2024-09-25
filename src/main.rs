use std::fs::{self};
// use std::fs::{self, DirEntry};

fn main() {
    let entries = fs::read_dir("D:/path/to/dir").unwrap();

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
