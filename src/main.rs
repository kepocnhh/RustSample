fn main() {
    for it in [Some(5), Some(42), None] {
        println!("{:?} + 1 = {:?}", it, plus_one(it));
    }
}

fn plus_one(it: Option<i32>) -> Option<i32> {
    match it {
        None => None,
        Some(i) => Some(i + 1),
    }
}
