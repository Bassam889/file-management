use std::fs::{self, read_to_string, File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() {
    //let choice: u32;
    //let filename: &str;
    //let content: &str;
    //let directory: &str;

    loop {
        println!("File Management System");
        println!("1. Create a file");
        println!("2. Write to a file");
        println!("3. Append to a file");
        println!("4. Read a file");
        println!("5. Delete a file");
        println!("6. List files");
        println!("7. Exit");
        print!("Enter your choice: ");

        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim().parse::<u32>() {
            Ok(1) => {
                println!("Enter filename: ");
                let mut filename = String::new();
                io::stdin().read_line(&mut filename).unwrap();
                create_file(&filename);
            }
            Ok(2) => {
                println!("Enter filename: ");
                let mut filename = String::new();
                io::stdin().read_line(&mut filename).unwrap();
                println!("Enter content to write: ");
                let mut content = String::new();
                io::stdin().read_line(&mut content).unwrap();
                write_to_file(&filename, &content);
            }
            Ok(3) => {
                println!("Enter filename: ");
                let mut filename = String::new();
                io::stdin().read_line(&mut filename).unwrap();
                println!("Enter content to write: ");
                let mut content = String::new();
                io::stdin().read_line(&mut content).unwrap();
                append_to_file(&filename, &content);
            }
            Ok(4) => {
                println!("Enter filename: ");
                let mut filename = String::new();
                io::stdin().read_line(&mut filename).unwrap();
                if let Ok(lines) = read_file(&filename) {
                    for line in lines.flatten() {
                        println!("{}", line);
                    }
                }
            }
            Ok(5) => {
                println!("Enter filename: ");
                let mut filename = String::new();
                io::stdin().read_line(&mut filename).unwrap();
                delete_file(&filename);
            }
            Ok(6) => {
                println!("Enter directory path: ");
                let mut directory = String::new();
                io::stdin().read_line(&mut directory).unwrap();
                list_files(&directory.trim());
            }
            Ok(7) => {
                println!("Exiting program...");
                break;
            }
            _ => {
                println!("Invalid choice, try again.");
            }
        }
    }

    //println!("1. Create a file");
    //println!("1. Create a file");
    //println!("1. Create a file");

    //create_file("poem.txt");
    //write_to_file("poem.txt", "Hello World");
    //if let Ok(lines) = read_file("poem.txt") {
    //    for line in lines.flatten() {
    //        println!("{}", line);
    //    }
    //}
    // delete_file("poem.txt");
    // list_files("/home/bassam/personal-tech-projects/low-level-dev/file_management_in_rust/");
}

fn create_file(filename: &str) {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(error) => panic!("could not create {}: {}", filename, error),
        Ok(_) => println!("File {} was created successfully.", filename),
    };
}

fn write_to_file(filename: &str, content: &str) {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match OpenOptions::new().write(true).create(false).open(&path) {
        Err(error) => panic!("could not open {}: {}", display, error),
        Ok(file) => {
            println!("File {} was found", display);
            file
        }
    };

    match file.write_all(content.as_bytes()) {
        Err(error) => panic!("couldn't write to {}: {}", display, error),
        Ok(file) => {
            println!("successfully wrote to {}", display);
            file
        }
    };
}

fn read_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
    //let mut result = Vec::new();
    //
    //for line in read_to_string(filename).unwrap().lines() {
    //    result.push(line.to_string())
    //}
    //
    //result

    //read_to_string(filename)
    //    .unwrap()
    //    .lines()
    //    .map(String::from)
    //    .collect()
}

fn append_to_file(filename: &str, content: &str) {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match OpenOptions::new()
        .append(true)
        .write(false)
        .create(false)
        .open(&path)
    {
        Err(error) => panic!("could not open {}: {}", display, error),
        Ok(file) => {
            println!("File {} was found", display);
            file
        }
    };
    match file.write_all(content.as_bytes()) {
        Err(error) => panic!("couldn't append to {}: {}", display, error),
        Ok(file) => {
            println!("successfully appended to {}", display);
            file
        }
    };
}

fn delete_file(filename: &str) {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match fs::remove_file(&path) {
        Err(error) => panic!("could not delete {}: {}", display, error),
        Ok(file) => {
            println!("successfully deleted {}", display);
            file
        }
    };
}
fn list_files(directory: &str) {
    let path = Path::new(directory);
    let display = path.display();

    if !path.is_dir() {
        println!("The specified path is not a directory or does not exist.");
        return;
    }

    println!("\nFiles in directory '{}': ", directory);

    let mut files = match fs::read_dir(&path) {
        Err(error) => panic!("could not list directory {}: {}", display, error),
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        if let Some(file_name) = entry.file_name().to_str() {
                            println!("- {}", file_name);
                        }
                    }
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
        }
    };
}
