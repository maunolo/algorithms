pub fn two_crystal_balls(breaks: &[bool]) -> Option<usize> {
    let jmp_amount = (breaks.len() as f32).sqrt().floor() as usize;

    let mut lo = 0;
    for i in (jmp_amount..breaks.len()).step_by(jmp_amount) {
        if breaks[i] {
            lo = i - jmp_amount;
            break;
        }
    }

    let mut hi = lo + jmp_amount;
    if hi >= breaks.len() {
        hi = breaks.len() - 1;
    }

    for i in lo..=hi {
        if breaks[i] {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_crystal_balls() {
        let v = vec![
            false, false, false, false, false, false, false, true, true, true,
        ];
        assert_eq!(two_crystal_balls(&v), Some(7));
        let v = vec![
            false, false, false, false, false, false, false, false, false, true,
        ];
        assert_eq!(two_crystal_balls(&v), Some(9));
        let v = vec![
            false, false, false, false, false, false, false, false, false, false,
        ];
        assert_eq!(two_crystal_balls(&v), None);
        let v = vec![true, true, true, true, true, true, true, true, true, true];
        assert_eq!(two_crystal_balls(&v), Some(0));
        let v = vec![false, false, true, true, true, true, true, true, true, true];
        assert_eq!(two_crystal_balls(&v), Some(2));
        let v = vec![
            false, false, false, false, true, true, true, true, true, true,
        ];
        assert_eq!(two_crystal_balls(&v), Some(4));
    }
}
