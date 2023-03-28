pub fn run() {
    const CHAPTER: u8 = 18;
    const PART: u8 = 3;
    const TITLE: &str = "Pattern Syntax";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _01();
    _02();
    _03();
    _04();
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

struct Foo {
    x: u8,
    y: u8
}

fn _02() {
    println!("\nDestructuring to Break Apart Values");

    let x: (_, _) = (1, 2);
    let foo = Foo { x: 1, y: 2 };
    let Foo { x: a, y: b } = foo;
    assert_eq!(foo.x, a);
    assert_eq!(foo.y, b);
    println!("x: {}, y: {}", foo.x, foo.y);
    println!("a: {a}, b: {b}");

    let Foo { x, y } = foo;
    assert_eq!(foo.x, x);
    assert_eq!(foo.y, y);
    println!("x: {}, y: {}", foo.x, foo.y);
    println!("x: {x}, y: {y}");

    match foo {
        Foo { x: 0, y } => println!("x is 0"),
        Foo { x, y: 0 } => println!("y is 0"),
        Foo { x, y } => println!("x: {x}, y: {y}"),
    }
}

enum Bar {
    Unit,
    U8(u8),
    Bool(bool),
    Str(String),
    Tuple { left: i32, right: String },
    Inner(Foo),
    Triple(u8, u8, u8),
}

trait Calculator {
    fn add(a: i32, b: i32) -> i32;
}

struct _180302 {}

impl Calculator for _180302 {
    fn add(_: i32, b: i32) -> i32 {
        println!("ignore a");
        return b
    }
}

fn _03() {
    println!("\nIgnoring Values in a Pattern");

    let bar = Bar::U8(42);
    match bar {
        Bar::U8(0) => println!("zero"),
        Bar::U8(_) => println!("any u8"),
        Bar::Bool(false) => println!("false"),
        Bar::Bool(_) => println!("any bool"),
        Bar::Str(_) => println!("any string"),
        Bar::Tuple { left: -1, right: _ } => println!("negative"),
        Bar::Tuple { .. } => println!("any tuple"),
        Bar::Unit => println!("unit"),
        Bar::Inner(Foo { x: 0, .. }) => println!("x is 0"),
        Bar::Inner(Foo { y: 0, .. }) => println!("y is 0"),
        Bar::Inner(Foo { x, y } ) => println!("x: {x}, y: {y}"),
        Bar::Triple(first, .., last) => println!("first: {first}, last: {last}"),
    }

    let foo = Some("foo");
    let bar = Some("bar");
    match (foo, bar) {
        (Some(_), Some(_)) => println!("Some"),
        _ => println!("any"),
    }
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    let _x = 5;
    let _y = 10;

    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("some any");
    }
    // if let Some(value) = s {
    //     println!("some '{value}' (borrow)");
    // }
    // if let Some(_ignored) = s {
    //     println!("some ignored value (borrow)");
    // }
    if let Some(_ignored) = &s {
        println!("some ignored value (not borrow)");
    }
    if let None = s {
        println!("None");
    }
    println!("{s:?}");
}

fn _04() {
    println!("\nExtra Conditionals with Match Guards");

    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => println!("None"),
    }

    match 4 {
        x if x % 2 == 0 => println!("The number {x} is even"),
        x => println!("The number {x} is odd"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("some 50"),
        Some(it) if it == y => println!("some {y}"),
        Some(_) => println!("some any"),
        None => println!("none"),
    }

    let x = 4;
    let yes_or_no = false;
    match x {
        4 | 5 | 6 if yes_or_no => println!("yes({x})"),
        _ if yes_or_no => println!("yes"),
        _ => println!("no"),
    }
}
