pub fn bubble_sort(arr: &mut [i32]) {
    let lenght = arr.len();
    for i in 0..lenght {
        for j in 0..lenght - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![1, 2, 3, 4, 5, 10, 7, 8, 9, 6];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let mut v = vec![1, 2, 11, 4, 5, 20, 7, 16, 9, 10];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 4, 5, 7, 9, 10, 11, 16, 20]);
    }
}
