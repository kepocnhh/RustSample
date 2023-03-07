use std::thread;
use std::time::Duration;

pub fn run() {
    const CHAPTER: u8 = 13;
    const PART: u8 = 1;
    const TITLE: &str = "Closures: Anonymous Functions that Capture Their Environment";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    closure_type_inference_and_annotation();
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
