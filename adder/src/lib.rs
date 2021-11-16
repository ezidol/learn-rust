pub fn plus_ten(n:i32) -> i32 {
    n + 10
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_plus_ten() {
        assert_ne!(plus_ten(10), 11);
    }
}