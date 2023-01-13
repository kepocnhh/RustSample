pub fn run() {
    const CHAPTER: u8 = 11;
    const PART: u8 = 2;
    const TITLE: &str = "Controlling How Tests Are Run";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    test_help();
}

fn test_help() {
    let commands = [
        "cargo test -- --test-threads=1",
        "cargo test -- --show-output",
        "cargo test {test_name}",
        "cargo test {test_name_prefix}",
        "cargo test -- --ignored",
    ];
    for (index, it) in commands.iter().enumerate() {
        println!("{index}] {it}")
    }
}

