pub fn run() {
    const CHAPTER: u8 = 9;
    const PART: u8 = 1;
    const TITLE: &str = "Unrecoverable Errors with panic!";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    panic();
    read_vec();
}

fn panic() {
    println!("panic!(\"crash and burn\")");
    // panic!("crash and burn");
}

fn read_vec() {
    println!("index out of bounds: the len is 3 but the index is 99");
    // vec![1, 2, 3][99];
}
