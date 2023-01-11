pub fn run() {
    const CHAPTER: u8 = 10;
    const PART: u8 = 3;
    const TITLE: &str = "Validating References with Lifetimes";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    preventing_dangling();
    generic_lifetimes();
    in_struct_definitions();
}

fn preventing_dangling() {
    println!("\nPreventing Dangling References with Lifetimes");

    preventing_dangling_0();
    preventing_dangling_1();
}

fn preventing_dangling_0() {
    let r;                // ---------+-- 'r
    {                     //          |
        let x = 5;        // -+-- 'x  |
        // r = &x;        //  |       | // borrow checker error
        r = x;            //  |       |
    }                     // -+       |
    println!("r: {}", r); //          |
}                         // ---------+

fn preventing_dangling_1() {
    let x = 5;            // ----------+-- 'x
                          //           |
    let r = &x;           // --+-- 'r  |
                          //   |       |
    println!("r: {}", r); //   |       |
}                         // --+-------+

fn generic_lifetimes() {
    println!("\nGeneric Lifetimes in Functions");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is \"{}\"", result);

    let r;
    let a = "abcd";
    {
        let b = "xyz";
        r = longest(a, b);
        // let b = String::from("xyz");
        // r = longest(a, b.as_str()); // error ?
    }
    println!("r \"{r}\"");

    let r;
    let a = 1;
    {
        // let b = 2;
        // r = max(&a, &b); // error
        r = max(&a, &3);
    }
    println!("r \"{r}\"");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn max<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn in_struct_definitions() {
    println!("\nLifetime Annotations in Struct Definitions");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let expert = Expert { part: first_sentence };
    in_struct_definitions_0(&expert);
}

struct Expert<'a> {
    part: &'a str,
}

fn in_struct_definitions_0(expert: &Expert) {
    println!("expert of \"{}\"", expert.part);
}
