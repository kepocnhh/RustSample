use std::str::FromStr;

pub fn run() {
    const CHAPTER: u8 = 19;
    const PART: u8 = 3;
    const TITLE: &str = "Advanced Types";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _01();
    _02();
    _03();
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
}

fn fun1<T: std::fmt::Display>(it: T) {
    println!("fun regular: {it}");
}

fn fun2<T: Sized + std::fmt::Display>(it: T) {
    println!("fun sized: {it}");
}

fn fun3<T: ?Sized + std::fmt::Display>(it: &T) {
    println!("fun optional sized: {it}");
}

trait Foo: std::fmt::Display {
    fn run(&self);
}

impl Foo for u8 {
    fn run(&self) {
        println!("i am u8 Foo");
    }
}

fn wrap<'a>(it: &'a u8) -> &'a dyn Foo {
    return it;
}

fn _03() {
    println!("\nDynamically Sized Types and the Sized Trait");
    let it = wrap(&1);
    // let it: &dyn Foo = &1u8;
    fun1(it);
    let it = wrap(&1);
    fun2(it);
    let it = wrap(&1);
    fun3(&it);
    // todo ?
}
