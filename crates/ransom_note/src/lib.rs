pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let letters = magazine.chars().fold(
        std::collections::HashMap::<char, u32>::new(),
        |mut letters, char| {
            letters
                .entry(char)
                .and_modify(|entry| *entry += 1)
                .or_insert(1);
            letters
        },
    );

    ransom_note
        .chars()
        .fold(
            (true, letters),
            |result_and_letters, char| match result_and_letters {
                (false, _) => result_and_letters,
                (true, mut letters) => match letters.entry(char) {
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        *entry.get_mut() -= 1;
                        if *entry.get() == 0 {
                            entry.remove_entry();
                        }
                        (true, letters)
                    }
                    std::collections::hash_map::Entry::Vacant(_) => (false, letters),
                },
            },
        )
        .0
}

#[cfg(test)]
mod ransom_note {

    #[test]
    fn example1() {
        let ransom_note = String::from("a");
        let magazine = String::from("b");
        let result = super::can_construct(ransom_note, magazine);
        assert_eq!(result, false);
    }

    #[test]
    fn example2() {
        let ransom_note = String::from("aa");
        let magazine = String::from("ab");
        let result = super::can_construct(ransom_note, magazine);
        assert_eq!(result, false);
    }

    #[test]
    fn example3() {
        let ransom_note = String::from("aa");
        let magazine = String::from("aab");
        let result = super::can_construct(ransom_note, magazine);
        assert_eq!(result, true);
    }
}
