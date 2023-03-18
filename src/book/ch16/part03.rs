use std::sync::mpsc;
use std::time::{Duration, SystemTime};

pub fn run() {
    const CHAPTER: u8 = 16;
    const PART: u8 = 3;
    const TITLE: &str = "Shared-State Concurrency";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _0101();
    todo!();
}

fn _0101() {
    println!("\nThe API of Mutex<T>");

    todo!();
}
