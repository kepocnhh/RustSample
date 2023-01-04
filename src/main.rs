fn main() {
    for it in [Some(1), Some(42), None] {
        match it {
            Some(value) => println!("match Some: {}", value),
            _ => (),
        }
        if let Some(value) = it {
            println!("if let Some: {}", value)
        }
        match it {
            Some(value) => println!("match Some: {}", value),
            _ => println!("match else: {:?}", it)
        }
        if let Some(value) = it {
            println!("if let Some: {}", value)
        } else {
            println!("if let else: {:?}", it)
        }
    }
}
