use rust_sample::book::ch1101;
use rust_sample::book::ch1101::U8Ex;

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
        assert!(!a.is_greater_than(b))
    }
    for it in [(2, 1), (42, 1), (255, 1)] {
        let (a, b) = it;
        assert!(a.is_greater_than(b))
    }
}
