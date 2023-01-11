use std::fmt::{Debug, Display, Formatter};

pub fn run() {
    const CHAPTER: u8 = 10;
    const PART: u8 = 2;
    const TITLE: &str = "Traits: Defining Shared Behavior";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    defining_a_trait();
    traits_as_parameters();
    trait_bound_syntax();
    multiple_trait();
    returning_traits();
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

fn traits_as_parameters() {
    println!("\nTraits as Parameters");

    let foo = Foo { value: "bar".to_string() };
    notify(&foo);
    // notify(&"foo".to_string()); // error
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn trait_bound_syntax() {
    println!("\nTrait Bound Syntax");

    let foo = Foo { value: "bar".to_string() };
    let bar = Bar { value: 42 };
    println_0(&foo, &bar);
    println_1(&foo, &foo);
}

struct Bar {
    value: u8,
}

impl Summary for Bar {
    fn summarize(&self) -> String {
        format!("value: {}", self.value)
    }
}

fn println_0(s1: &impl Summary, s2: &impl Summary) {
    println!("0: {}\n1: {}", s1.summarize(), s2.summarize());
}

fn println_1<T: Summary>(s1: &T, s2: &T) {
    println!("0: {}\n1: {}", s1.summarize(), s2.summarize());
}

fn multiple_trait() {
    println!("\nSpecifying Multiple Trait Bounds with the + Syntax");

    let foo = Foo { value: "bar".to_string() };
    display_0(&foo);
    // display(&Bar { value: 42 }); // error
    display_1(&foo);
    multiple_trait_0(&foo, &"foo".to_string());
}

fn display_0(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

fn display_1<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn multiple_trait_0<T, U>(t: &T, u: &U)
    where
        T: Display,
        U: Debug {
    println!("display: {}\ndebug: {:?}", t, u);
}

fn returning_traits() {
    println!("\nReturning Types that Implement Traits");

    let summary = returning_traits_0();
    println!("returning summary: {}", summary.summarize())
}

fn returning_traits_0() -> impl Summary {
    Foo { value: "foo".to_string() }
}
