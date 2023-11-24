pub fn sum(set: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for count in set {
        sum = sum.checked_add(*count)?
    }
    Some(sum)
}

mod test {

    #[test]
    fn test_sum() {
        use super::sum;
        let set = [u32::MAX, 10, 23];
        let sum = sum(&set);
        assert_eq!(sum, None);
    }
}
