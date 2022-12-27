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
}
