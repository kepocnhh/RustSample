pub fn run() {
    const CHAPTER: u8 = 10;
    const PART: u8 = 1;
    const TITLE: &str = "Recoverable Errors with Result";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    in_function_definitions();
    in_struct_definitions();
    in_method_definitions();
}

fn in_function_definitions() {
    println!("\nIn Function Definitions");

    let list = vec![34, 50, 25, 100, 65];
    let result = largest(&list);
    println!("The largest number is {result}");
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn in_struct_definitions() {
    println!("\nIn Struct Definitions");

    let point = Point { x: 5, y: 10 };
    println!("{:?}", point);
    let point = Point { x: 1.0, y: 4.0 };
    println!("{:?}", point);
    println!("Point(x:{}, y:{})", point.x, point.y);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn in_method_definitions() {
    println!("\nIn Method Definitions");

    let point = Point { x: 1.0_f32, y: 4.0_f32 };
    let distance_from_origin = point.distance_from_origin();
    println!("distance from origin: {distance_from_origin}");
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
