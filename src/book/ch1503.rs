pub fn run() {
    const CHAPTER: u8 = 15;
    const PART: u8 = 3;
    const TITLE: &str = "Running Code on Cleanup with the Drop Trait";
    println!("\n\t{CHAPTER:02}/{PART:02}\t\"{TITLE}\"");

    _150301();
    _150302();
}

struct MyDrop(String);

impl Drop for MyDrop {
    fn drop(&mut self) {
        println!("Dropping with `{}`!", self.0);
    }
}

fn _150301() {
    println!("\nDropping a Value");

    let a = MyDrop(String::from("foo"));
    println!("Allocated a: {}", a.0);
    let b = MyDrop(String::from("bar"));
    println!("Allocated b: {}", b.0);
    println!("finish...");
}

fn _150302() {
    println!("\nDropping a Value Early with std::mem::drop");

    let a = MyDrop(String::from("foo"));
    println!("Allocated a: {}", a.0);
    drop(a);
    let b = MyDrop(String::from("bar"));
    println!("Allocated b: {}", b.0);
    println!("finish...");
}
