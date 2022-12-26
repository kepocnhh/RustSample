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

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;
        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        number -= 1;
    }
    println!("number: {number}");

    let array = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value {index} is: {}", array[index]);
        index += 1;
    }

    for it in array {
        println!("the value {index} is: {it}");
    }

    for number in 1..4 {
        println!("number: {number}");
    }
    for number in (1..4).rev() {
        println!("reversed: {number}");
    }
}
