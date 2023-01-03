fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, [2, 3]);
    assert_ne!(slice, [1, 2]);
    assert_eq!(a[1..3], [2, 3]);
    assert_ne!(a[1..3], a[1..4]);
    let item = slice.get(1).expect("TODO");
    assert_eq!(item, &3)
}
