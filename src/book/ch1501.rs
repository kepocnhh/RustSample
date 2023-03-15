pub fn run() {
    const CHAPTER: u8 = 15;
    const PART: u8 = 1;
    const TITLE: &str = "Using Box<T> to Point to Data on the Heap";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _150101();
    _150102();
}

fn _150101() {
    println!("\nUsing a Box<T> to Store Data on the Heap");

    let value = Box::new(5);
    println!("value: {value}");
}


#[derive(Debug)]
enum Ints {
    Cons { value: i32, next: Box<Ints> },
    Nil,
}

fn _150102() {
    println!("\nEnabling Recursive Types with Boxes");

    // let list = Ints::Cons {
    //     value: 1,
    //     next: Box::new(
    //         Ints::Cons {
    //             value: 2,
    //             next: Box::new(Ints::Nil),
    //         }
    //     ),
    // };
    let list = Ints::Cons {
        value: 1,
        next: Ints::Cons {
            value: 2,
            next: Ints::Nil.boxed(),
        }.boxed()
    };
    println!("value: {list:?}");
}

trait Boxed {
    fn boxed(self) -> Box<Self>;
}

impl <T> Boxed for T {
    fn boxed(self) -> Box<Self> {
        return Box::new(self)
    }
}
