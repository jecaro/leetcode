pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = strs
        .iter()
        .fold(std::collections::BTreeMap::new(), |mut acc, word| {
            let letters =
                word.chars()
                    .fold(std::collections::BTreeMap::new(), |mut letters, char| {
                        letters
                            .entry(char)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                        letters
                    });
            acc.entry(letters)
                .and_modify(|words: &mut Vec<String>| {
                    words.push(word.clone());
                    words.sort();
                })
                .or_insert(vec![word.to_string()]);
            acc
        })
        .into_iter()
        .map(|(_key, value)| value)
        .collect();

    result.sort_by_key(|x| x.len());
    result
}

#[cfg(test)]
mod group_anagrams {

    #[test]
    fn example1() {
        let input = vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ];
        let result = super::group_anagrams(input);
        assert_eq!(
            result,
            vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
        );
    }

    #[test]
    fn example2() {
        let input = vec![String::from("")];
        let result = super::group_anagrams(input);
        assert_eq!(result, vec![vec![""]]);
    }

    #[test]
    fn example3() {
        let input = vec![String::from("a")];
        let result = super::group_anagrams(input);
        assert_eq!(result, vec![vec!["a"]]);
    }
}
