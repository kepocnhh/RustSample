pub fn run() {
    const CHAPTER: u8 = 13;
    const PART: u8 = 2;
    const TITLE: &str = "Processing a Series of Items with Iterators";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _130201();
    todo!();
}

fn _130201() {
    println!("\nThe Iterator Trait and the next Method");

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
    todo!();
}
