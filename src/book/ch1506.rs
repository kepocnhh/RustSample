use std::cell::RefCell;
use std::rc::Rc;

pub fn run() {
    const CHAPTER: u8 = 15;
    const PART: u8 = 6;
    const TITLE: &str = "Reference Cycles Can Leak Memory";
    println!("\n\t{CHAPTER:02}/{PART:02}\t\"{TITLE}\"");

    _150601();
    todo!();
}

#[derive(Debug)]
enum TailedInts {
    Cons { value: i32, next: RefCell<Rc<TailedInts>> },
    Nil,
}

impl TailedInts {
    fn tail(&self) -> Option<&RefCell<Rc<TailedInts>>> {
        return match self {
            TailedInts::Cons { value: _, next } => Some(next),
            TailedInts::Nil => None
        }
    }
}

fn _150601() {
    println!("\nCreating a Reference Cycle");

    let a = Rc::new(
        TailedInts::Cons {
            value: 1,
            next: RefCell::new(Rc::new(TailedInts::Nil))
        }
    );
    println!("tail before: {:?}", a.tail());
    let b = Rc::new(
        TailedInts::Cons {
            value: 1,
            next: RefCell::new(Rc::clone(&a))
        }
    );
    if let Some(next) = a.tail() {
        *next.borrow_mut() = Rc::clone(&b);
    }
    // println!("tail after: {:?}", a.tail()); // out of available memory
}
