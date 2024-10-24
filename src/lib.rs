pub fn multiply(x: &i32, y: &i32) -> i32 {
    x * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(&10, &20), 200);
    }
}
