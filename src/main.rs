fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("color: {}", black.1);
    println!("point: {}", origin.1);
    let subject = AlwaysEqual;
    // assert_eq!(subject, subject)
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;
