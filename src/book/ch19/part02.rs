use std::ops::Add;

pub fn run() {
    const CHAPTER: u8 = 19;
    const PART: u8 = 2;
    const TITLE: &str = "Advanced Traits";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _01();
    _02();
    todo!();
}

trait Foo {
    type T;
    fn get_any(&self) -> &Self::T;
}

trait Bar<T> {
    fn get_any(&self) -> &T;
}

impl Foo for Vec<u8> {
    type T = u8;

    fn get_any(&self) -> &Self::T {
        return self.get(0).unwrap();
    }
}

impl Bar<String> for Vec<String> {
    fn get_any(&self) -> &String {
        return self.get(0).unwrap();
    }
}

fn _01() {
    println!("\nSpecifying Placeholder Types in Trait Definitions with Associated Types");

    let foo = vec![1, 2, 3];
    let any = foo.get_any();
    println!("foo any: {any}");

    let bar = vec![String::from("baz")];
    let any = bar.get_any();
    println!("bar any: {any}");
}

struct MyU8(u8);

impl Add for MyU8 {
    type Output = MyU8;
    fn add(self, rhs: Self) -> Self::Output {
        return MyU8(self.0 + rhs.0)
    }
}

impl Add<u8> for MyU8 {
    type Output = MyU8;

    fn add(self, rhs: u8) -> Self::Output {
        return MyU8(self.0 + rhs)
    }
}

fn _02() {
    println!("\nDefault Generic Type Parameters and Operator Overloading");

    let m1 = MyU8(2);
    let m2 = MyU8(3);
    let m3: u8 = 4;
    assert_eq!(m1.0 + m2.0, 5);
    assert_eq!(m2.0 + m3, 7);
    println!("m1: {}, m2: {}, m3:{m3}", m1.0, m2.0);
}
