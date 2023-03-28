pub fn run() {
    const CHAPTER: u8 = 18;
    const PART: u8 = 3;
    const TITLE: &str = "Pattern Syntax";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _01();
    todo!();
}

fn _01() {
    println!("\nMatching Literals");

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // shadowed y
        // None => println!("None"), // or default
        _ => println!("Default case, x = {x:?}"),
    }

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 42;
    match x {
        1..=100 => println!("1..=100"),
        _ => println!("something else"),
    }
}
