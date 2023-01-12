use rust_sample::book::ch1101;
use rust_sample::book::ch1101::{panic_if_one, U8Ex};

#[test]
pub fn max_test() {
    for it in [(1, 2), (1, 42), (1, 255)] {
        let (a, b) = it;
        assert_ne!(a, b);
        let result = ch1101::max(a, b);
        assert_eq!(result, b)
    }
    for it in [(2, 1), (42, 1), (255, 1)] {
        let (a, b) = it;
        assert_ne!(a, b);
        let result = ch1101::max(a, b);
        assert_eq!(result, a)
    }
    let a = 1;
    let result = ch1101::max(a, a);
    assert_eq!(result, a)
}

#[test]
pub fn is_greater_than_test() {
    for it in [(1, 2), (1, 42), (1, 255)] {
        let (a, b) = it;
        assert!(
            !a.is_greater_than(b),
            "\"a\" is greater than \"b\"!"
        )
    }
    for it in [(2, 1), (42, 1), (255, 1)] {
        let (a, b) = it;
        assert!(
            a.is_greater_than(b),
            "\"b\" is greater than \"a\"!"
        )
    }
}

#[test]
pub fn panic_if_one_test_no_panic() {
    for it in [42, 255] {
        panic_if_one(it);
    }
}

#[test]
#[should_panic(expected = "Value is one!")]
pub fn panic_if_one_test() {
    panic_if_one(1);
}
