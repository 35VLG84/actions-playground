pub fn ht() -> String {
    "Hello there".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ht_test() {
        assert_eq!(ht(), "Hello there".to_string());
    }
}
