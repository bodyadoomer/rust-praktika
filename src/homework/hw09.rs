fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }
    
    let n = (n.rem_euclid(len as isize)) as usize;
    let (left, right) = s.split_at(len - n);
    format!("{}{}", right, left)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rotate() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        for &(n, expected) in &shifts {
            assert_eq!(rotate(s.clone(), n), expected.to_string());
        }
    }
}
