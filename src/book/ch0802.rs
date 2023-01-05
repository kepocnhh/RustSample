pub fn run() {
    println!("\nStoring UTF-8 Encoded Text with Strings");
    new_string()
}

fn new_string() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("string: {s}");
    s.push('l');
    println!("string: {s}");
}
