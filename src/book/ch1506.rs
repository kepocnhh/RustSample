use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub fn run() {
    const CHAPTER: u8 = 15;
    const PART: u8 = 6;
    const TITLE: &str = "Reference Cycles Can Leak Memory";
    println!("\n\t{CHAPTER:02}/{PART:02}\t\"{TITLE}\"");

    _150601();
    _150602();
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
        };
    }
}

fn _150601() {
    println!("\nCreating a Reference Cycle");

    let a = Rc::new(
        TailedInts::Cons {
            value: 1,
            next: RefCell::new(Rc::new(TailedInts::Nil)),
        }
    );
    println!("tail before: {:?}", a.tail());
    let b = Rc::new(
        TailedInts::Cons {
            value: 1,
            next: RefCell::new(Rc::clone(&a)),
        }
    );
    if let Some(next) = a.tail() {
        *next.borrow_mut() = Rc::clone(&b);
    }
    // println!("tail after: {:?}", a.tail()); // out of available memory
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[derive(Debug)]
enum WeakInts {
    Cons {
        name: String,
        value: i32,
        next: RefCell<Weak<WeakInts>>
    },
    Nil,
}

impl WeakInts {
    fn tail(&self) -> Option<&RefCell<Weak<WeakInts>>> {
        return match self {
            WeakInts::Cons { name: _, value: _, next } => Some(next),
            WeakInts::Nil => None
        };
    }
}

fn _150602() {
    println!("\nPreventing Reference Cycles: Turning an Rc<T> into a Weak<T>");

    let leaf = Rc::new(
        Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    );
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    let branch = Rc::new(
        Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        }
    );
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());


    let a = Rc::new(
        WeakInts::Cons {
            name: String::from("a"),
            value: 1,
            next: RefCell::new(Weak::new()),
        }
    );
    match a.tail() {
        None => {
            println!("tail before: none");
        }
        Some(ref_cell) => {
            println!("tail before: {:?}", ref_cell.borrow().upgrade());
        }
    }
    println!("a strong count: {}", Rc::strong_count(&a));
    println!("a weak count: {}", Rc::weak_count(&a));
    let b = Rc::new(
        WeakInts::Cons {
            name: String::from("b"),
            value: 1,
            next: RefCell::new(Rc::downgrade(&a)),
        }
    );
    println!("init b...");
    println!("a strong count: {}", Rc::strong_count(&a));
    println!("a weak count: {}", Rc::weak_count(&a));
    println!("b strong count: {}", Rc::strong_count(&b));
    println!("b weak count: {}", Rc::weak_count(&b));
    if let Some(next) = a.tail() {
        *next.borrow_mut() = Rc::downgrade(&b);
    }
    println!("set b...");
    println!("a strong count: {}", Rc::strong_count(&a));
    println!("a weak count: {}", Rc::weak_count(&a));
    println!("b strong count: {}", Rc::strong_count(&b));
    println!("b weak count: {}", Rc::weak_count(&b));
    match a.tail() {
        None => {
            println!("tail after: none");
        }
        Some(ref_cell) => {
            println!("tail after: {:?}", ref_cell.borrow().upgrade());
        }
    }
}
