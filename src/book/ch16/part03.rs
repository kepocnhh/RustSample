use std::sync::{Arc, Mutex};

pub fn run() {
    const CHAPTER: u8 = 16;
    const PART: u8 = 3;
    const TITLE: &str = "Shared-State Concurrency";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _0301();
}

fn _0301() {
    println!("\nSharing a Mutex<T> Between Multiple Threads");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
