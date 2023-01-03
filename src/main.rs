fn main() {
    let word = first_word("hello world");
    println!("first word: \"{}\"", word);

    let text = String::from("hello world");
    let word = first_word(&text);
    println!("first word: \"{}\"", word);
    let word = first_word(&text[0..text.len()]);
    println!("first word: \"{}\"", word);
    let word = first_word(&text[..text.len()]);
    println!("first word: \"{}\"", word);
    let word = first_word(&text[0..]);
    println!("first word: \"{}\"", word);
    let word = first_word(&text[..]);
    println!("first word: \"{}\"", word);

    let text = "hello world";
    let word = first_word(text);
    println!("first word: \"{}\"", word);
    let word = first_word(&text[0..text.len()]);
    println!("first word: \"{}\"", word);
    let word = first_word(&text[..text.len()]);
    println!("first word: \"{}\"", word);
    let word = first_word(&text[0..]);
    println!("first word: \"{}\"", word);
    let word = first_word(&text[..]);
    println!("first word: \"{}\"", word);
}

fn first_word(text: &str) -> &str {
    for (i, &item) in text.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &text[..i];
        }
    }
    return text
}
