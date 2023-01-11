pub fn run() {
    const CHAPTER: u8 = 10;
    const PART: u8 = 3;
    const TITLE: &str = "Validating References with Lifetimes";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    preventing_dangling()
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
