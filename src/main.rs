fn main() {
    let s = String::from("hello world");
    let slice = &s[0..5];
    println!("slice [0..5]: \"{}\"", slice);
    let slice = &s[6..11];
    println!("slice [6..11]: \"{}\"", slice);
    let slice = &s[..5];
    println!("slice [..5]: \"{}\"", slice);
    let len = s.len();
    let slice = &s[6..len];
    println!("slice [6..len]: \"{}\"", slice);
    let slice = &s[6..s.len()];
    println!("slice [6..s.len()]: \"{}\"", slice);
    let slice = &s[6..];
    println!("slice [6..]: \"{}\"", slice);
    let slice = &s[0..len];
    println!("slice [0..len]: \"{}\"", slice);
    let slice = &s[..];
    println!("slice [..]: \"{}\"", slice);
}
