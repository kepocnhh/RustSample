use std::fs::File;
use std::io::ErrorKind;

pub fn run() {
    const CHAPTER: u8 = 9;
    const PART: u8 = 2;
    const TITLE: &str = "Recoverable Errors with Result";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    open_file();
    matching_errors();
}

fn open_file() {
    let name = "foo.txt";
    match File::open(name) {
        Ok(file) => println!("File: {:?}", file),
        Err(error) => println!("Error: file \"{name}\" {:?}", error),
    };
}

fn matching_errors() {
    println!("\nMatching on Different Errors");

    let name = "foo.txt";
    match File::open(name) {
        Ok(file) => println!("open file: {:?}", file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(name) {
                Ok(file) => println!("create file: {:?}", file),
                Err(error) => println!("create file \"{name}\" error: {:?}", error)
            },
            _ => println!("open file \"{name}\" error: {:?}", error)
        }
    };
}
