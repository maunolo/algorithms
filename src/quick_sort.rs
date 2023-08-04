fn qs(arr: &mut [i32], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let p = partition(arr, lo, hi);
    qs(arr, lo, p - 1);
    qs(arr, p + 1, hi);
}

fn partition(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let mut idx = lo as isize - 1;

    for i in lo..hi {
        if arr[i] <= arr[hi] {
            idx += 1;
            arr.swap(idx as usize, i);
        }
    }

    idx += 1;
    arr.swap(idx as usize, hi);

    idx as usize
}

pub fn quick_sort(arr: &mut [i32]) {
    qs(arr, 0, arr.len() - 1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quick_sort_test() {
        let mut v = vec![9, 3, 7, 4, 69, 420, 42];
        quick_sort(&mut v);
        assert_eq!(v, vec![3, 4, 7, 9, 42, 69, 420]);
    }
}
