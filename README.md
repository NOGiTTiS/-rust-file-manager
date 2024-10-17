# Directory Listing Program

This Rust program lists all files and directories in a specified path. If no path is provided, it defaults to `C:/`.

## Features

- Collects command-line arguments to specify the directory path.
- Lists all files and directories in the specified path.
- Provides a default path if none is specified.

## Usage

1. **Clone the repository** (if applicable):

   ```bash
   git clone <repository-url>
   cd <repository-directory>
   ```

2. **Build the program**:

   ```bash
   cargo build --release
   ```

3. **Run the program**:

   ```bash
   ./target/release/your_program_name <optional-path>
   ```

   Replace `<optional-path>` with the path you want to list. If you do not provide a path, the program will use `C:/` as the default.

## Example

To list files in `C:/Users`:
