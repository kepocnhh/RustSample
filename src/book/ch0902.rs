use std::fs::File;

pub fn run() {
    const CHAPTER: u8 = 9;
    const PART: u8 = 2;
    const TITLE: &str = "Recoverable Errors with Result";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    open_file();
}

fn open_file() {
    let name = "foo.txt";
    match File::open(name) {
        Ok(file) => println!("File: {:?}", file),
        Err(error) => println!("Error: file \"{name}\" {:?}", error),
    };
}
