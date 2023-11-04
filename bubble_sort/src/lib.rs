//
pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    for _ in 0..arr.len() {
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut a = [4, 7, 2, 9, 11, 1, 3, 5, 8, 6, 10];
        let b = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        bubble_sort(&mut a);
        assert_eq!(a, b);
    }
}
