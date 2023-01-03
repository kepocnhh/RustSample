fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rectangle is {:?}", rectangle);
    println!("rectangle width {}", rectangle.get_width());

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );

    let r1 = Rectangle { width: 10, height: 40 };
    let r2 = Rectangle { width: 40, height: 40 };
    println!("Can hold? {}", rectangle.can_hold(&r1));
    println!("Can hold? {}", rectangle.can_hold(&r2));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
