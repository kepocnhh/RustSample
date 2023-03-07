pub fn run() {
    const CHAPTER: u8 = 13;
    const PART: u8 = 2;
    const TITLE: &str = "Processing a Series of Items with Iterators";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _130201();
    _130202();
    _130203();
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

fn _130202() {
    println!("\nMethods that Consume the Iterator");

    let list = vec![1, 2, 3];
    let list_iter = list.iter();
    let total: i32 = list_iter.sum();
    println!("sum of {list:?} is {total}");

    let list: Vec<_> = list.iter().map(|x| x + 1).collect();
    println!("mapped: {list:?}");
}

fn _130203() {
    println!("\nUsing Closures that Capture Their Environment");

    let list = vec![1, 2, 3, 4, 42];
    let a = 2;
    let b = 0;
    let list: Vec<_> = list.iter().filter(|it| *it % a == b).collect();
    println!("filtered: {list:?}");
}

fn type_name<T>(_: T) -> &'static str {
    return std::any::type_name::<T>();
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
            return None;
        }
        *self += 1;
        return Some(*self);
    }
}

impl Foo42<u8> for u8 {
    fn next42(&mut self) -> Option<u8> {
        if self == &u8::MAX {
            return None;
        }
        *self += 1;
        return Some(*self);
    }
}
