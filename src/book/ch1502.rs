use std::ops::Deref;

pub fn run() {
    const CHAPTER: u8 = 15;
    const PART: u8 = 2;
    const TITLE: &str = "Treating Smart Pointers Like Regular References with the Deref Trait";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _150201();
    _150202();
    todo!();
}

fn _150201() {
    println!("\nFollowing the Pointer to the Value");

    let value = 5;
    let foo = &value;
    let bar = Box::new(value);

    assert_eq!(5, value);
    assert_eq!(value, *foo);
    assert_eq!(value, *bar);
    println!("value: {value}, reference: {}, Box<i32>: {}", *foo, *bar);
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn _150202() {
    println!("\nDefining Our Own Smart Pointer");

    let value = 5;
    let foo = MyBox(value);

    assert_eq!(5, value);
    assert_eq!(value, *foo);
    println!("value: {value}, MyBox<i32>: {}", *foo);
}
