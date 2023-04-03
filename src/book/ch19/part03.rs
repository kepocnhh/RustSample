use std::num::ParseIntError;
use std::str::FromStr;

pub fn run() {
    const CHAPTER: u8 = 19;
    const PART: u8 = 3;
    const TITLE: &str = "Advanced Types";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _01();
    _02();
    todo!();
}

type Kilometers = i32;
type BoxedFun = Box<dyn Fn() + Send + 'static>;
type Fun = fn();
type Result<T> = std::result::Result<T, std::num::ParseIntError>;

fn _01() {
    println!("\nCreating Type Synonyms with Type Aliases");

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let foo: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("foo"));
    let bar: BoxedFun = Box::new(|| println!("bar"));
    let baz: Fun = || println!("baz");
    foo();
    bar();
    baz();

    let result: Result<u8> = "1".parse::<u8>();
    println!("result: {result:?}");
    let result: Result<u8> = u8::from_str("1");
    println!("result: {result:?}");
}

fn never_fun() -> ! {
    panic!("never!");
}

fn _02() {
    println!("\nThe Never Type that Never Returns");

    let num: u8 = match "1".parse::<u8>() {
        Ok(it) => it,
        Err(_) => never_fun()
    };
    println!("num: {num}");
    todo!();
}
