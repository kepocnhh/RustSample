fn main() {
    let text = String::from("foo bar baz");
    let i = first_word(&text);
    println!("first word: {}", i);
}

fn first_word(s: &String) -> usize {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
