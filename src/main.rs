fn main() {
    const CONST_STR: &str = "foo";
    let s0 = CONST_STR;
    let s1 = CONST_STR;
    let let_str: &str = "bar";
    let s0 = let_str;
    let s1 = let_str;
    let let_string = String::from("baz");
    let s0 = let_string; // Ownership is moved here
    // let s1 = let_string; // ERR: value used here after move

    // functions
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function
    // let s1 = s; // ERR: value used here after move

    // Return Values and Scope
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    return some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    return a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
