pub fn run() {
    const CHAPTER: u8 = 19;
    const PART: u8 = 4;
    const TITLE: &str = "Advanced Functions and Closures";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _01();
    _02();
}

fn map(it: i32, transform: fn(i32) -> i32) -> i32 {
    return transform(it);
}

fn double(it: &i32) -> i32 {
    return it * 2;
}

#[derive(Debug)]
enum Foo {
    Bar(i32)
}

fn _01() {
    println!("\nFunction Pointers");

    let initial = 1;
    let transformed = map(initial, |it| it * 2);
    assert_eq!(initial * 2, transformed);
    println!("initial: {initial}, transformed: {transformed}");

    let initial = vec![1, 2, 3, 4, 5];
    println!("initial: {initial:?}");
    let transformed: Vec<String> = initial.iter().map(|it| it.to_string()).collect();
    println!("transformed: {transformed:?}");
    let transformed: Vec<String> = initial.iter().map(ToString::to_string).collect();
    println!("transformed: {transformed:?}");
    let transformed: Vec<i32> = initial.iter().map(double).collect();
    println!("transformed: {transformed:?}");
    let transformed: Vec<Foo> = initial.iter().map(|it| Foo::Bar(*it)).collect();
    println!("transformed: {transformed:?}");
    let transformed: Vec<Foo> = initial.clone().into_iter().map(Foo::Bar).collect();
    println!("transformed: {transformed:?}");
    let transformed: Vec<Foo> = initial.iter().copied().map(Foo::Bar).collect();
    println!("transformed: {transformed:?}");
    let transformed: Vec<Foo> = (1i32..=5).map(Foo::Bar).collect();
    println!("transformed: {transformed:?}");
}

// fn returns_closure() -> dyn Fn(i32) -> i32 { // does not have a constant size known at compile-time
fn returns_closure_boxed() -> Box<dyn Fn(i32) -> i32> {
    return Box::new(|x| x + 1);
}

fn returns_closure() -> impl Fn(i32) -> i32 {
    return |x| x + 1;
}

fn transform(it: i32, block: impl Fn(i32) -> i32) -> i32 {
    return block(it);
}

fn regular_function(it: i32) -> i32 {
    return it + 1;
}

fn _02() {
    println!("\nReturning Closures");

    let initial = 1;
    println!("initial: {initial}");

    let transformed = transform(initial, returns_closure_boxed());
    assert_eq!(initial + 1, transformed);
    println!("transformed: {transformed}");

    let transformed = transform(initial, returns_closure());
    assert_eq!(initial + 1, transformed);
    println!("transformed: {transformed}");

    let closure = |it| it + 1;
    let transformed = transform(initial, closure);
    assert_eq!(initial + 1, transformed);
    println!("transformed: {transformed}");

    let transformed = transform(initial, regular_function);
    assert_eq!(initial + 1, transformed);
    println!("transformed: {transformed}");
}
