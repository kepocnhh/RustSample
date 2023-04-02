use std::ops::Add;

pub fn run() {
    const CHAPTER: u8 = 19;
    const PART: u8 = 2;
    const TITLE: &str = "Advanced Traits";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _01();
    _02();
    _03();
    _04();
    _05();
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

impl Add for &MyU8 {
    type Output = MyU8;
    fn add(self, rhs: Self) -> Self::Output {
        return MyU8(self.0 + rhs.0);
    }
}

impl Add<u8> for &MyU8 {
    type Output = MyU8;

    fn add(self, rhs: u8) -> Self::Output {
        return MyU8(self.0 + rhs);
    }
}

fn _02() {
    println!("\nDefault Generic Type Parameters and Operator Overloading");

    let m1 = MyU8(2);
    let m2 = MyU8(3);
    let m3: u8 = 4;
    assert_eq!((&m1 + &m2).0, 5);
    assert_eq!((&m2 + m3).0, 7);
    println!("m1: {}, m2: {}, m3:{m3}", m1.0, m2.0);
}

trait T1 {
    fn foo(&self);
    fn bar();
}

trait T2 {
    fn foo(&self);
    fn bar();
}

struct S1;

impl S1 {
    fn foo(&self) {
        println!("S1::foo")
    }

    fn bar() {
        println!("S1::bar")
    }
}

impl T1 for S1 {
    fn foo(&self) {
        println!("T1::foo")
    }

    fn bar() {
        println!("T1::bar")
    }
}

impl T2 for S1 {
    fn foo(&self) {
        println!("T2::foo")
    }

    fn bar() {
        println!("T2::bar")
    }
}

fn _03() {
    println!("\nFully Qualified Syntax for Disambiguation: Calling Methods with the Same Name");

    let s1 = S1;
    s1.foo();
    T1::foo(&s1);
    T2::foo(&s1);
    S1::bar();
    <S1 as T1>::bar();
    <S1 as T2>::bar();
}

trait SquarePrint: std::fmt::Display {
    fn square_print(&self) {
        let value = self.to_string();
        let len = value.len();
        println!("+{}+", "-".repeat(len + 2));
        println!("| {} |", value);
        println!("+{}+", "-".repeat(len + 2));
    }
}

impl SquarePrint for String {}

fn _04() {
    println!("\nUsing Supertraits to Require One Trait’s Functionality Within Another Trait");

    let foo = String::from("bar");
    foo.square_print();
}

struct Wrapper(Vec<String>);

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn _05() {
    println!("\nUsing the Newtype Pattern to Implement External Traits on External Types");

    let it = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("wrapped: {it}");
}
