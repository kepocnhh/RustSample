use std::time::Duration;

pub fn run() {
    const CHAPTER: u8 = 16;
    const PART: u8 = 1;
    const TITLE: &str = "Using Threads to Run Code Simultaneously";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _0101();
    _0102();
    _0103();
}

fn _0101() {
    println!("\nCreating a New Thread with spawn");

    std::thread::spawn(|| {
        let subpart = 1;
        let current = std::thread::current();
        let name = current.name().unwrap_or("spawned");
        for i in 1..10 {
            println!("{subpart:02}] number {i} from the \"{name}\" thread!");
            std::thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..3 {
        println!("hi number {i} from the {:?} thread!", std::thread::current().name());
        std::thread::sleep(Duration::from_millis(1));
    }
}

fn _0102() {
    println!("\nWaiting for All Threads to Finish Using join Handles");

    let handle = std::thread::spawn(|| {
        let subpart = 2;
        let current = std::thread::current();
        let name = current.name().unwrap_or("spawned");
        for i in 1..10 {
            println!("{subpart:02}] number {i} from the \"{name}\" thread!");
            std::thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..3 {
        println!("hi number {i} from the {:?} thread!", std::thread::current().name());
        std::thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn _0103() {
    println!("\nUsing move Closures with Threads");

    let v = vec![1, 2, 3];
    println!("Before vector: {v:?}");
    let handle = std::thread::spawn(move || {
        println!("Inside vector: {v:?}");
    });
    // println!("Outside vector: {v:?}"); // moved
    handle.join().unwrap();
}
