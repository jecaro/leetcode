pub fn length_of_longest_substring(s: String) -> i32 {
    let mut seen_chars = std::collections::HashSet::new();

    if let Some(char) = s.chars().nth(0) {
        seen_chars.insert(char);
        let mut longuest = 1;

        // left side of the window
        let mut itr = s.chars();
        // right side of the window
        for char in s.chars().skip(1) {
            if seen_chars.contains(&char) {
                while let Some(other) = itr.next() {
                    seen_chars.remove(&other);
                    if other == char {
                        break;
                    }
                }
            }
            seen_chars.insert(char);

            longuest = std::cmp::max(longuest, seen_chars.len());
        }
        longuest as i32
    } else {
        0
    }
}

#[cfg(test)]
mod longest_substring_without_repeating_characters {

    #[test]
    fn example1() {
        let s = String::from("abcabcbb");
        let result = super::length_of_longest_substring(s);
        assert_eq!(result, 3);
    }

    #[test]
    fn example2() {
        let s = String::from("bbbbb");
        let result = super::length_of_longest_substring(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn example3() {
        let s = String::from("pwwkew");
        let result = super::length_of_longest_substring(s);
        assert_eq!(result, 3);
    }

    #[test]
    fn example4() {
        let s = String::from("aab");
        let result = super::length_of_longest_substring(s);
        assert_eq!(result, 2);
    }

    #[test]
    fn example5() {
        let s = String::from("aabaab!bb");
        let result = super::length_of_longest_substring(s);
        assert_eq!(result, 3);
    }

    #[test]
    fn example6() {
        let s = String::from(" ");
        let result = super::length_of_longest_substring(s);
        assert_eq!(result, 1);
    }
}
