pub fn run() {
    const CHAPTER: u8 = 19;
    const PART: u8 = 1;
    const TITLE: &str = "Unsafe Rust";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _01();
    todo!();
}

fn _01() {
    println!("\nDereferencing a Raw Pointer");

    let r0 = &16;
    println!("r0 pointer: {r0}");
    println!("r0 dereferenced: {}", *r0);

    let mut foo = 32;
    // let r1 = &foo; let mut r2 = &mut foo; println!("{r1}"); // cannot borrow as mutable because it is also borrowed as immutable
    let r1 = &foo as *const i32;
    let r2 = &mut foo as *mut i32;

    println!("r1 pointer: {r1:?}");
    println!("r2 pointer: {r2:?}");

    unsafe {
        println!("r1 dereferenced: {}", *r1);
        println!("r2 dereferenced: {}", *r2);
    }

    let address = 0x012345usize;
    let r3 = address as *const i32;
    println!("r3 pointer: {r3:?}");

    unsafe {
        // println!("r3 dereferenced: {}", *r3); // segmentation fault
    }
}
