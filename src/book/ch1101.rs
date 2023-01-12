pub fn run() {
    const CHAPTER: u8 = 11;
    const PART: u8 = 1;
    const TITLE: &str = "How to Write Tests";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    print_max();
}

fn print_max() {
    println!("\nThe Anatomy of a Test Function");

    let a = 1;
    let b = 2;
    let max = max(a, b);
    println!("max: {max}");
}

pub fn max(a: u8, b: u8) -> u8 {
    if a < b {
        b
    } else {
        a
    }
}

pub trait U8Ex {
    fn is_greater_than(&self, other: u8) -> bool;
}

impl U8Ex for u8 {
    fn is_greater_than(&self, other: u8) -> bool {
        self > &other
    }
}

pub fn panic_if_one(value: u8) {
    if value == 1 {
        panic!("Value is one!");
    }
    println!("Value is not one. OK.");
}

pub fn error_if_one(value: u8) -> Result<(), String> {
    if value == 1 {
        return Err("Value is one!".to_owned());
    }
    return Ok(());
}
