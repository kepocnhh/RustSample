use std::fmt::{Display, Formatter};

pub fn run() {
    const CHAPTER: u8 = 10;
    const PART: u8 = 2;
    const TITLE: &str = "Traits: Defining Shared Behavior";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    defining_a_trait();
}

fn defining_a_trait() {
    println!("\nDefining a Trait");

    let foo = Foo { value: "bar".to_string() };
    println!("foo: {foo}");
    println!("summary: {}", foo.summarize());
}

trait Summary {
    fn summarize(&self) -> String;
}

struct Foo {
    value: String,
}

impl Summary for Foo {
    fn summarize(&self) -> String {
        format!("value: {}", self.value)
    }
}

impl Display for Foo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Foo({})", self.value)
    }
}
