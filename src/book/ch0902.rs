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
    shortcut_for_propagating();
    read_to_string();
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

fn shortcut_for_propagating() {
    println!("\nA Shortcut for Propagating");

    match ok_or_error_shortcut() {
        Ok(it) => println!("ok: {:?}", it),
        Err(error) => println!("error: {:?}", error)
    }
}

fn ok_or_error_shortcut() -> Result<String, Error> {
    let name = "foo.txt";
    // let file = File::open(name)?;
    // let metadata = file.metadata()?;
    // return Ok(format!("is_file: {}", metadata.is_file()));
    let is_file = File::open(name)
        ?.metadata()
        ?.is_file();
    return Ok(format!("is_file: {}", is_file));
}

fn read_to_string() {
    println!("\nstd::fs::read_to_string");

    let name = "foo.txt";
    let value = "foobar";
    match std::fs::write(name, value) {
        Ok(_) => match std::fs::read_to_string(name) {
            Ok(it) => assert_eq!(value, it),
            Err(error) => println!("read error: {:?}", error)
        }
        Err(error) => println!("write error: {:?}", error)
    }
}
