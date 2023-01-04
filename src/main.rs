fn main() {
    let some_number = Some(5);
    println!("{:?}", some_number);
    let some_char = Some('e');
    println!("{:?}", some_char);
    let none_number: Option<i32> = None;
    println!("{:?}", none_number);
}
