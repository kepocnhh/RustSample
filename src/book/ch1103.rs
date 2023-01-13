pub fn run() {
    const CHAPTER: u8 = 11;
    const PART: u8 = 3;
    const TITLE: &str = "Test Organization";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    private_function();
}

fn private_function() {
    println!("Private function!");
}

#[cfg(test)]
mod tests {
    use super::private_function;

    #[test]
    fn private_function_test() {
        private_function();
    }
}
