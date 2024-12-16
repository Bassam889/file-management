# File Management System in Rust

This is a simple command-line application built in Rust
that provides basic file management operations. It allows
users to create, write to, append to, read from, delete files,
and list all files in a specified directory.

## Features

1. Create a File:

- Users can create a new file by providing the desired filename.

2. Write to a File:

- Allows writing content to a file, overwriting any existing content.

3. Append to a File:

- Adds content to the end of an existing file.

4. Read from a File:

- Reads and displays the content of a specified file.

5. Delete a File:

- Deletes a specified file from the system.

6. List Files in a Directory:

- Lists all files and subdirectories within a given directory.

7. Exit:

- Exits the application.

## Prerequisites

Rust installed on your system. You can install Rust using rustup.

## How to run:

1. Clone the repository or copy the source code into a local directory.

```bash
git clone <repository-url>
cd <repository-folder>
````

2. Build the project using Cargo:

```bash
cargo build
```

3. Run the application:

```bash
cargo run
```

4. Follow the on-screen prompts to perform file management operations.

## Directory Listing Example

When listing files in a directory, the output will look like this:

```bash
Enter directory path: /path/to/directory

Files in directory '/path/to/directory':
- file1.txt
- file2.rs
- subdirectory
- README.md
```

## Error Handling

- The program handles invalid paths gracefully and notifies the user if a specified file or - directory does not exist.

- Proper error messages are displayed if operations such as file creation, reading, or deletion fail.

## Project Structure

The project is contained in a single file for simplicity:

- main.rs: Contains the implementation of all file management functions 
and the main program loop.

## Code Breakdown

## Key Functions

- create_file: Creates a new file.

- write_to_file: Writes (overwrites) content to a file.

- append_to_file: Appends content to a file.

- read_file: Reads content from a file line by line.

- delete_file: Deletes a specified file.

- list_files: Lists all files and subdirectories in a directory.

## Example Code Snippet: Listing Files

```rs
fn list_files(directory: &str) {
  let path = Path::new(directory);

  if !path.is_dir() {
    println!("The specified path is not a directory or does not exist.");
    return;
  }
  
  println!("\nFIles in directory '{}':", direcotry);

  match fs::read_dir(&path) {
    Err(error) => eprintln!("Error: Could not list directory '{}': {}", directory)
    Ok(entries) => {
      for entry in entries {
        match entry {
          Ok(entry) => {
            if let Some(file_name) = entry.file_name().to_str() {
              println!("- {}", file_name);
            }
          }
          Err(error) => eprintln!("Error reading an entry: {}", error),
        }
      }
    }
  }
  println!();
}
```

## Future Improvements

- Add support for file renaming.

- Include a search functionality within directories.

- Display file sizes and metadata when listing files.

- Add better user input validation.

## License

This project is open-source and available under the MIT License. Feel free to use, modify, and distribute it as per the terms of the license.

--------------------------------------------------------------------------------------

Built with Rust for learning and experimentation!
