pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
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
    fn test_reverse() {
        assert_eq!(reverse("Data Structures"), "serutcurtS ataD");
        assert_eq!(reverse("Rust"), "tsuR");
        assert_eq!(reverse(""), "");
        assert_eq!(reverse("A"), "A");
    }
}
