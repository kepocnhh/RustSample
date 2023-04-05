pub fn run() {
    const CHAPTER: u8 = 19;
    const PART: u8 = 5;
    const TITLE: &str = "Macros";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _01();
    _02();
}

#[macro_export]
macro_rules! num_args {
    () => {
        {
            println!("no args");
            0
        }
    };
    ($it:expr) => {
        {
            println!("single: {}", $it);
            1
        }
    };
    ($($it:expr),*) => {
        {
            let mut number = 0usize;
            $(
                println!("arg[{number}]: {}", $it);
                number += 1;
            )*
            number
        }
    };
}

fn _01() {
    println!("\nDeclarative Macros with macro_rules! for General Metaprogramming");

    let number = num_args!();
    println!("number: {number}");
    let number = num_args!(42);
    println!("number: {number}");
    let number = num_args!(1, 2, 3);
    println!("number: {number}");
    let number = num_args!(1, "foo", false);
    println!("number: {number}");
}

fn _02() {
    println!("\nHow to Write a Custom derive Macro");
    // todo foo_derive?
    // todo Attribute-like macros?
    // todo Function-like macros?
}
