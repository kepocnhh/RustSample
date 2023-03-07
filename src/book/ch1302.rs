pub fn run() {
    const CHAPTER: u8 = 13;
    const PART: u8 = 2;
    const TITLE: &str = "Processing a Series of Items with Iterators";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _130201();
    todo!();
}

fn _130201() {
    println!("\nThe Iterator Trait and the next Method");

    let list = vec![1, 2, 3];
    let list_iter = list.iter();
    for it in list_iter {
        println!("Got: {}", it);
    }

    let mut it = 42u8;
    println!("next of {it} is {:?}", it.next());
    let mut it = u8::MAX;
    println!("next of {it} is {:?}", it.next42());

    let mut list_iter = list.iter();
    assert_eq!(list_iter.next(), Some(&1));
    assert_eq!(list_iter.next(), Some(&2));
    assert_eq!(list_iter.next(), Some(&3));
    assert_eq!(list_iter.next(), None);

    for it in list.iter() {
        println!("Got: {} type {}", it, type_name(it));
    }
    for it in list.into_iter() {
        println!("Got: {} type {}", it, type_name(it));
    }
    for it in vec![1, 2, 3].iter_mut() {
        let value = it.clone();
        let type_name = type_name(it);
        println!("Got: {} type {}", value, type_name);
    }
}

fn type_name<T>(_: T) -> &'static str {
    return std::any::type_name::<T>()
}

trait Foo {
    type Bar;

    fn next(&mut self) -> Option<Self::Bar>;
}

trait Foo42<Bar> {
    fn next42(&mut self) -> Option<Bar>;
}

impl Foo for u8 {
    type Bar = u8;

    fn next(&mut self) -> Option<Self::Bar> {
        if self == &u8::MAX {
            return None
        }
        *self += 1;
        return Some(*self);
    }
}

impl Foo42<u8> for u8 {
    fn next42(&mut self) -> Option<u8> {
        if self == &u8::MAX {
            return None
        }
        *self += 1;
        return Some(*self);
    }
}
