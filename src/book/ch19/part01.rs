pub fn run() {
    const CHAPTER: u8 = 19;
    const PART: u8 = 1;
    const TITLE: &str = "Unsafe Rust";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _01();
    _02();
    _03();
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

unsafe fn unsafefn() {
    println!("unsafe function");
}

fn my_split_at_mut(list: &mut [u32], mid: usize) -> (&mut [u32], &mut [u32]) {
    let len = list.len();
    assert!(mid <= len);
    let ptr = list.as_mut_ptr();
    return unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    };
    // return (&mut list[..mid], &mut list[mid..]); // cannot borrow as mutable more than once at a time
}

fn _02() {
    println!("\nCalling an Unsafe Function or Method");

    unsafe {
        unsafefn();
    }

    let initial = [1, 0, 3, 0, 5, 6];
    let mid = 2;

    let mut list = initial.clone();
    let reference = &mut list;
    let (left, right) = reference.split_at_mut(mid);
    assert_eq!(left, [1, 0]);
    assert_eq!(right, [3, 0, 5, 6]);
    left[1] = 2;
    right[1] = 4;
    assert_eq!(reference, &[1, 2, 3, 4, 5, 6]);
    println!("split_at_mut: {reference:?}");

    let mut list = initial.clone();
    let reference = &mut list;
    let (left, right) = my_split_at_mut(reference, mid);
    assert_eq!(left, [1, 0]);
    assert_eq!(right, [3, 0, 5, 6]);
    left[1] = 2;
    right[1] = 4;
    assert_eq!(reference, &[1, 2, 3, 4, 5, 6]);
    println!("my_split_at_mut: {reference:?}");
}

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn _03() {
    println!("\nAccessing or Modifying a Mutable Static Variable");

    println!("HELLO_WORLD: {HELLO_WORLD}");

    // Use of mutable static is unsafe and requires unsafe function or block
    unsafe {
        COUNTER += 1;
        println!("COUNTER: {COUNTER}");
    }
}

// todo unsafe trait
// todo union
