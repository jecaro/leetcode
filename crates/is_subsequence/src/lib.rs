pub fn is_subsequence(s: String, t: String) -> bool {
    let mut t_itr = t.chars();

    for s_char in s.chars() {
        let found = loop {
            match t_itr.next() {
                Some(t_char) => {
                    if t_char == s_char {
                        break true;
                    }
                }
                None => break false,
            }
        };
        if !found {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod is_subsequence {

    #[test]
    fn example1() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");
        let result = super::is_subsequence(s, t);
        assert_eq!(result, true);
    }

    #[test]
    fn example2() {
        let s = String::from("axc");
        let t = String::from("ahbgdc");
        let result = super::is_subsequence(s, t);
        assert_eq!(result, false);
    }
}
