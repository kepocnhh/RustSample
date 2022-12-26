fn main() {
    // if Expressions
    let number = 3;
    if number < 5 {
        println!("{} < {}", number, 5);
    } else {
        println!("{} >= {}", number, 5);
    }

    let number = 6;
    if number < 5 {
        println!("{} < {}", number, 5);
    } else {
        println!("{} >= {}", number, 5);
    }

    if number % 4 == 0 {
        println!("{number} is divisible by 4");
    } else if number % 3 == 0 {
        println!("{number} is divisible by 3");
    } else if number % 2 == 0 {
        println!("{number} is divisible by 2");
    } else {
        println!("{number} is not divisible by 4, 3, or 2");
    }

    // if in a let Statement
    let condition = true;
    let integer = if condition { 5 } else { 6 };
    println!("integer is {integer}");
}
