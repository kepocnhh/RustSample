fn main() {
    let s = String::from("foo");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);

    let mut ms = String::from("bar");
    change(&mut ms);
    println!("Mutable: '{}'", ms);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
