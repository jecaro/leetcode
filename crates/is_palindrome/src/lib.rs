pub fn is_palindrome(s: String) -> bool {
    let sanitized = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<Vec<char>>();
    for i in 0..sanitized.len() / 2 {
        if sanitized[i] != sanitized[sanitized.len() - 1 - i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod is_palindrome {

    #[test]
    fn example1() {
        let s = String::from("A man, a plan, a canal: Panama");
        let result = super::is_palindrome(s);
        assert_eq!(result, true);
    }

    #[test]
    fn example2() {
        let s = String::from("race a car");
        let result = super::is_palindrome(s);
        assert_eq!(result, false);
    }
}
