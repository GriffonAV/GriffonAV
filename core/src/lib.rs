pub fn scan(path: &str) -> String {
    format!("Scanned file: {}", path)
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_critique() {
        assert_eq!(2 + 2, 4);
    }

    #[cfg(feature = "experimental")]
    #[test]
    fn test_experimental() {
        assert_eq!(2 + 2, 5); // volontairement faux
    }
}
