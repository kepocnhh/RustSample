pub fn run() {
    println!("\nStoring UTF-8 Encoded Text with Strings");

    new_string();
    concatenation();
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
