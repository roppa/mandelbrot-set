#[cfg(test)]
mod tests {
    use crate::formula;
    #[test]
    fn test_formula() {
        assert_eq!(formula(0, 0), 0);
    }
}

pub fn formula(z: i64, c: i64) -> i64 {
    return z.pow(2) + c;
}
