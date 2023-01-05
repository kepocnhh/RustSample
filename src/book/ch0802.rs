pub fn run() {
    const CHAPTER: u8 = 8;
    const PART: u8 = 2;
    const TITLE: &str = "Storing UTF-8 Encoded Text with Strings";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    new_string();
    concatenation();
    indexing();
}

fn new_string() {
    println!("\nCreating a New String");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("string: {s}");
    s.push('l');
    println!("string: {s}");
}

fn concatenation() {
    println!("\nConcatenation with the + Operator or the format! Macro");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s = s1 + &s2;
    println!("string: {s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("string: {s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("string: {s}");
}

fn indexing() {
    println!("\nIndexing into Strings");

    let s = "Здравствуйте";
    let value = 4;
    println!("string \"{s}\" -> 0..{value} = \"{}\"", &s[..value]);
    // println!("string: {}", &s[0..1]); // error
}
