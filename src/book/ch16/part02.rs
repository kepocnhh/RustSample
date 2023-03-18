use std::sync::mpsc;
use std::time::{Duration, SystemTime};

pub fn run() {
    const CHAPTER: u8 = 16;
    const PART: u8 = 2;
    const TITLE: &str = "Using Message Passing to Transfer Data Between Threads";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _0101();
    _0102();
    _0103();
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

fn _0102() {
    println!("\nChannels and Ownership Transference");

    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        for it in 1..10 {
            tx.send(it.to_string()).unwrap();
            std::thread::sleep(Duration::from_millis(128));
        }
    });

    for it in rx {
        println!("it: {it}");
    }
}

fn _0103() {
    println!("\nSending Multiple Values and Seeing the Receiver Waiting");

    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();

    std::thread::spawn(move || {
        let number = 1;
        for it in (number * 10 + 1)..(number * 10 + 10) {
            let message = format!("from[{number}]: {it}");
            tx1.send(message).unwrap();
            std::thread::sleep(Duration::from_millis(128));
        }
    });

    std::thread::spawn(move || {
        let number = 2;
        for it in (number * 10 + 1)..(number * 10 + 10) {
            let message = format!("from[{number}]: {it}");
            tx2.send(message).unwrap();
            std::thread::sleep(Duration::from_millis(256));
        }
    });

    for it in rx {
        println!("{it}");
    }
}
