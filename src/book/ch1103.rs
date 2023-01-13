pub fn run() {
    const CHAPTER: u8 = 11;
    const PART: u8 = 3;
    const TITLE: &str = "Test Organization";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    public_function();
    private_function();
}

pub fn public_function() {
    println!("Public function.");
}

fn private_function() {
    println!("Private function!");
}

#[cfg(test)]
#[path = "ch1103_unit_tests.rs"]
mod internal_tests_outside;

#[cfg(test)]
mod internal_tests_inside {
    #[test]
    fn private_function_test() {
        super::private_function();
    }
}
