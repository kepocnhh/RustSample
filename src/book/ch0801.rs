pub fn new_vec() {
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
