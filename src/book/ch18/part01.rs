pub fn run() {
    const CHAPTER: u8 = 18;
    const PART: u8 = 1;
    const TITLE: &str = "All the Places Patterns Can Be Used";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _01();
    todo!();
}

fn _01() {
    println!("\nmatch Arms");

    let initial = 42;
    let option = Some(initial);
    let incremented = match option {
        None => 0,
        Some(value) => value + 1
    };
    assert_eq!(initial + 1, incremented);
    println!("incremented({incremented}) = initial({initial}) + 1");
}
