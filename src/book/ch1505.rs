use std::cell::RefCell;
use std::rc::Rc;

pub fn run() {
    const CHAPTER: u8 = 15;
    const PART: u8 = 5;
    const TITLE: &str = "RefCell<T> and the Interior Mutability Pattern";
    println!("\n\t{CHAPTER:02}/{PART:02}\t\"{TITLE}\"");

    _150501();
    _150502();
    _150503();
}

trait Foo {
    fn send(&self, msg: &str);
}

struct Bar<'a, T: Foo> {
    foo: &'a T,
    value: u8,
}

impl<'a, T> Bar<'a, T> where T: Foo {
    fn send(&mut self) {
        self.value += 1;
        let msg = format!("value: {}", self.value);
        self.foo.send(&msg)
    }
}

struct MockFoo {
    msgs: RefCell<Vec<String>>
}

impl Foo for MockFoo {
    fn send(&self, msg: &str) {
        self.msgs.borrow_mut().push(String::from(msg));
    }
}

fn _150501() {
    println!("\nEnforcing Borrowing Rules at Runtime with RefCell<T>");

    let initial = 0;
    let foo = MockFoo { msgs: RefCell::new(vec![]) };
    let mut bar = Bar {
        foo: &foo,
        value: initial
    };
    bar.send();
    let expected = format!("value: {}", initial + 1);
    let actual = foo.msgs.borrow()[0].clone();
    assert_eq!(actual, expected);
    println!("Expected is \"{expected}\", actual is \"{actual}\".");
}

fn _150502() {
    println!("\nKeeping Track of Borrows at Runtime with RefCell<T>");

    let foo = MockFoo { msgs: RefCell::new(vec![]) };
    let a1 = foo.msgs.borrow_mut();
    // let a2 = foo.msgs.borrow_mut(); // already borrowed: BorrowMutError
}

#[derive(Debug)]
enum RCInts {
    Cons { value: Rc<RefCell<i32>>, next: Box<RCInts> },
    Nil,
}

fn _150503() {
    println!("\nHaving Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>");

    let value = Rc::new(RefCell::new(42));
    let a = RCInts::Cons {
        value: Rc::clone(&value),
        next: Box::new(RCInts::Nil)
    };
    let b = RCInts::Cons {
        value: Rc::clone(&value),
        next: Box::new(RCInts::Nil)
    };
    let c = RCInts::Cons {
        value: Rc::clone(&value),
        next: Box::new(RCInts::Nil)
    };
    println!("before: a = {a:?}, b = {b:?}, c = {c:?}");
    *value.borrow_mut() += 1;
    println!("after: a = {a:?}, b = {b:?}, c = {c:?}");
}
