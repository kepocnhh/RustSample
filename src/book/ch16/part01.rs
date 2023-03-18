use std::time::Duration;

pub fn run() {
    const CHAPTER: u8 = 16;
    const PART: u8 = 1;
    const TITLE: &str = "Using Threads to Run Code Simultaneously";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _0101();
    todo!();
}

fn _0101() {
    println!("\nCreating a New Thread with spawn");

    std::thread::spawn(|| {
        let current = std::thread::current();
        let name = current.name().unwrap_or("spawned");
        for i in 1..10 {
            println!("hi number {i} from the \"{name}\" thread!");
            std::thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..3 {
        println!("hi number {i} from the {:?} thread!", std::thread::current().name());
        std::thread::sleep(Duration::from_millis(1));
    }
}
