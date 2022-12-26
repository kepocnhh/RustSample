fn main() {
    // mut
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // const
    const NUMBER_U8_MAX: u8 = u8::MAX;
    println!("NUMBER_U8_MAX: {NUMBER_U8_MAX}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // shadowing vs mut
    let spaces = "   ";
    let spaces = spaces.len();
}
