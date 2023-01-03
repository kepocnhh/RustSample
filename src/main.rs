fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("color: {}", black.1);
    println!("point: {}", origin.1);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
