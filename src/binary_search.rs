pub fn binary_search(haystack: &[i32], needle: i32) -> Option<usize> {
    let mut lo: f32 = 0 as f32;
    let mut hi: f32 = haystack.len() as f32;
    while lo < hi {
        let m = (lo + (hi - lo) / 2 as f32).floor() as usize;
        let value = haystack[m];

        if value == needle {
            return Some(m);
        } else if value > needle {
            hi = m as f32;
        } else {
            lo = (m + 1) as f32;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(binary_search(&v, 1), Some(0));
        assert_eq!(binary_search(&v, 2), Some(1));
        assert_eq!(binary_search(&v, 3), Some(2));
        assert_eq!(binary_search(&v, 4), Some(3));
        assert_eq!(binary_search(&v, 5), Some(4));
        assert_eq!(binary_search(&v, 6), Some(5));
        assert_eq!(binary_search(&v, 7), Some(6));
        assert_eq!(binary_search(&v, 8), Some(7));
        assert_eq!(binary_search(&v, 9), Some(8));
        assert_eq!(binary_search(&v, 10), Some(9));
        assert_eq!(binary_search(&v, 11), None);
        assert_eq!(binary_search(&v, 0), None);
    }
}
