use std::sync::mpsc;
use std::time::{Duration, SystemTime};

pub fn run() {
    const CHAPTER: u8 = 16;
    const PART: u8 = 2;
    const TITLE: &str = "Using Message Passing to Transfer Data Between Threads";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _0101();
    todo!();
}

fn _0101() {
    println!("\nWe create a new channel using the mpsc::channel function");

    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0));
        let current = std::thread::current();
        let name = current.name().unwrap_or(&now.as_millis().to_string()).to_owned();
        tx.send(name).unwrap();
    });

    match rx.recv() {
        Ok(name) => {
            println!("Name: {name}");
        }
        Err(error) => {
            println!("Error: {error}");
        }
    }
}
