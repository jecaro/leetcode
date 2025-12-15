pub fn is_valid(s: String) -> bool {
    let result = s.chars().fold(
        (true, Vec::<char>::new()),
        |(result, mut acc), char| match result {
            false => (false, acc),
            true => match char {
                '{' | '(' | '[' => {
                    acc.push(char);
                    (true, acc)
                }
                '}' | ')' | ']' => match acc.pop() {
                    None => (false, acc),
                    Some(opening) => (correspond(opening, char), acc),
                },
                _ => (false, acc),
            },
        },
    );

    result.0 && result.1.is_empty()
}

fn correspond(opening: char, closing: char) -> bool {
    (opening == '(' && closing == ')')
        || (opening == '[' && closing == ']')
        || (opening == '{' && closing == '}')
}

#[cfg(test)]
mod valid_parentheses {

    #[test]
    fn example1() {
        let s = String::from("()");
        let result = super::is_valid(s);
        assert_eq!(result, true);
    }

    #[test]
    fn example2() {
        let s = String::from("()[]{}");
        let result = super::is_valid(s);
        assert_eq!(result, true);
    }

    #[test]
    fn example3() {
        let s = String::from("(]");
        let result = super::is_valid(s);
        assert_eq!(result, false);
    }

    #[test]
    fn example4() {
        let s = String::from("([])");
        let result = super::is_valid(s);
        assert_eq!(result, true);
    }

    #[test]
    fn example5() {
        let s = String::from("([)]");
        let result = super::is_valid(s);
        assert_eq!(result, false);
    }
}
