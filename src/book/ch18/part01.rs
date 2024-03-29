pub fn run() {
    const CHAPTER: u8 = 18;
    const PART: u8 = 1;
    const TITLE: &str = "All the Places Patterns Can Be Used";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _01();
    _02();
    _03();
    _04();
    _05();
    _06();
}

fn _01() {
    println!("\nmatch Arms");

    let initial = 42;
    let option = Some(initial);
    let incremented = match option {
        None => 0,
        Some(value) => value + 1
    };
    assert_eq!(initial + 1, incremented);
    println!("incremented({incremented}) = initial({initial}) + 1");
}

fn _02() {
    println!("\nConditional if let Expressions");

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(value) = favorite_color {
        println!("Favorite color: {value}");
    } else if is_tuesday {
        println!("Is tuesday.");
    } else if let Ok(value) = age {
        println!("Age: {value}");
    } else {
        panic!("Illegal state!");
    }
}

fn _03() {
    println!("\nwhile let Conditional Loops");

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(value) = stack.pop() {
        println!("{value}");
    }
}

fn _04() {
    println!("\nfor Loops");

    for (index, value) in vec!['a', 'b', 'c'].iter().enumerate() {
        println!("{value} is at index {index}");
    }
}

fn _05() {
    println!("\nlet Statements");

    // let (x, y) = (1, 2, 3); // error
    let (x, y, _) = (1, 2, 3);
    let (x, y, z) = (1, 2, 3);
    // let (x, y, z) = (1, 2); // error
    println!("x: {x}, y: {y}, z: {z}");
}

fn on_tuple((left, right): (&str, &str)) {
    println!("left: {left}, right: {right}");
}

fn _06() {
    println!("\nFunction Parameters");

    let tuple = ("<", ">");
    on_tuple(tuple);
}
