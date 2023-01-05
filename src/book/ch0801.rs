pub fn run() {
    new_vec();
    get_item();
}

fn new_vec() {
    println!("\nCreating a New Vector");
    let v: Vec<i32> = Vec::new();
    println!("vector: {:?}", v);
    let v = vec![1, 2, 3];
    println!("vector: {:?}", v);
    let v = vec_of(&[1, 2, 3]);
    println!("vector: {:?}", v);
    let v = [1, 2, 3].to_vec();
    println!("vector: {:?}", v);
}

fn vec_of<T : Copy>(array: &[T]) -> Vec<T> {
    let mut result = Vec::new();
    for it in array {
        result.push(*it);
    }
    return result
}

trait ArrayExt<T : Copy> {
    fn to_vec(&self) -> Vec<T>;
}

impl<T : Copy> ArrayExt<T> for [T] {
    fn to_vec(&self) -> Vec<T> {
        vec_of(self)
    }
}

fn get_item() {
    println!("\nReading Elements of Vectors");
    // let mut vector = vec![1, 2, 3, 4, 5];
    // let item = &vector[0];
    // vector.push(1); // error
    // println!("item: {item}");
    let vector = vec![1, 2, 3, 4, 5];
    let index = 2;
    let item: i32 = vector[index];
    println!("The [{index}] element is {item}");
    let item: Option<&i32> = vector.get(index);
    match item {
        Some(item) => println!("The [{index}] element is {item}"),
        None => println!("There is no [{index}] element."),
    }
    let index = 42;
    // let item: i32 = vector[index];
    // println!("The [{index}] element is {item}");
    match vector.get(index) {
        Some(item) => println!("The [{index}] element is {item}"),
        None => println!("There is no [{index}] element."),
    }
}
