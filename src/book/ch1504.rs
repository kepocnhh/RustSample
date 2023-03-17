use std::rc::Rc;

pub fn run() {
    const CHAPTER: u8 = 15;
    const PART: u8 = 4;
    const TITLE: &str = "Rc<T>, the Reference Counted Smart Pointer";
    println!("\n\t{CHAPTER:02}/{PART:02}\t\"{TITLE}\"");

    _150401();
    todo!();
}

#[derive(Debug)]
enum BoxInts {
    Cons { value: i32, next: Box<BoxInts> },
    Nil,
}

#[derive(Debug)]
enum RcInts {
    Cons { value: i32, next: Rc<RcInts> },
    Nil,
}

fn _150401() {
    println!("\nUsing Rc<T> to Share Data");

    let a = BoxInts::Cons {
        value: 1,
        next: Box::new(
            BoxInts::Cons {
                value: 2,
                next: Box::new(BoxInts::Nil)
            }
        )
    };
    let b = BoxInts::Cons {
        value: 3,
        next: Box::new(a)
    };

    let a = RcInts::Cons {
        value: 1,
        next: Rc::new(
            RcInts::Cons {
                value: 2,
                next: Rc::new(RcInts::Nil)
            }
        )
    };
    let rc = Rc::new(a);
    let b = RcInts::Cons {
        value: 3,
        next: Rc::clone(&rc)
    };
    let c = RcInts::Cons {
        value: 4,
        next: Rc::clone(&rc)
    };
    println!("a: {rc:?}");
    println!("b: {b:?}");
    println!("c: {c:?}");
}
