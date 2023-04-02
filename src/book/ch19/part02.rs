pub fn run() {
    const CHAPTER: u8 = 19;
    const PART: u8 = 2;
    const TITLE: &str = "Advanced Traits";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _01();
    todo!();
}

trait Foo {
    type T;
    fn get_any(&self) -> &Self::T;
}

trait Bar<T> {
    fn get_any(&self) -> &T;
}

impl Foo for Vec<u8> {
    type T = u8;

    fn get_any(&self) -> &Self::T {
        return self.get(0).unwrap();
    }
}

impl Bar<String> for Vec<String> {
    fn get_any(&self) -> &String {
        return self.get(0).unwrap();
    }
}

fn _01() {
    println!("\nSpecifying Placeholder Types in Trait Definitions with Associated Types");

    let foo = vec![1, 2, 3];
    let any = foo.get_any();
    println!("foo any: {any}");

    let bar = vec![String::from("baz")];
    let any = bar.get_any();
    println!("bar any: {any}");
}
