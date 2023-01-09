use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;

pub fn run() {
    const CHAPTER: u8 = 9;
    const PART: u8 = 2;
    const TITLE: &str = "Recoverable Errors with Result";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    open_file();
    matching_errors();
    unwrap_and_expect();
    propagating_errors();
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

fn unwrap_and_expect() {
    println!("\nShortcuts for Panic on Error: unwrap and expect");

    let name = "foo.txt";
    match File::open(name) {
        Ok(file) => println!("open file: {:?}", file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(name) {
                Ok(file) => println!("create file: {:?}", file),
                Err(error) => panic!("create file \"{name}\" error: {:?}", error)
            },
            _ => panic!("open file \"{name}\" error: {:?}", error)
        }
    };
    let file = File::open(name).unwrap();
    println!("Unwrap: {:?}", file);
    let file = File::open(name).expect("foo.txt should be included in this project");
    println!("Expect: {:?}", file);
}

fn propagating_errors() {
    println!("\nPropagating Errors");

    match ok_or_error() {
        Ok(it) => println!("ok: {:?}", it),
        Err(error) => println!("error: {:?}", error)
    }
}

fn ok_or_error() -> Result<String, Error> {
    let name = "foo.txt";
    return match File::open(name) {
        Ok(_) => Ok(name.to_string()),
        Err(error) => Err(error)
    };
}
