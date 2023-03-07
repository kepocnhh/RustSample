use std::thread;
use std::time::Duration;

pub fn run() {
    const CHAPTER: u8 = 13;
    const PART: u8 = 1;
    const TITLE: &str = "Closures: Anonymous Functions that Capture Their Environment";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    closure_type_inference_and_annotation();
    capturing_references_or_moving_ownership();
    todo!();
}

fn closure_type_inference_and_annotation() {
    println!("\nClosure Type Inference and Annotation");

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    };
    let result = expensive_closure(42);
    println!("Result is {result}");

    fn add_one_v1(x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;

    let x = 42;
    println!("v1: x + 1 = {}", add_one_v1(x));
    println!("v2: x + 1 = {}", add_one_v2(x));
    println!("v3: x + 1 = {}", add_one_v3(x));
    println!("v4: x + 1 = {}", add_one_v4(x));

    let example_closure = |x| x;
    let result = example_closure(String::from("hello"));
    // let n = example_closure(5); // expected struct `String`, found integer
    println!("Example closure. Result: {}", result);
}

fn capturing_references_or_moving_ownership() {
    println!("\nCapturing References or Moving Ownership");

    let mut list = vec![1, 2, 3];
    let immutable_list = list.clone();

    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", immutable_list);
    only_borrows();

    let mut borrows_mutably = || list.push(7);
    borrows_mutably();

    println!("After calling closure mutable: {:?}", list);
    println!("After calling closure: {:?}", immutable_list);

    println!("Before defining closure: {:?}", list);

    let handle = thread::current();
    let name = handle.name().expect("Get thread name error!");
    println!("thread: {name}");
    thread::Builder::new()
        .name("foo thread".into())
        .spawn(move || {
            let handle = thread::current();
            let name = handle.name().expect("Get thread name error!");
            println!("thread: {name}");
            println!("From thread: {:?}", list)
        }).unwrap()
        .join()
        .unwrap();
}
