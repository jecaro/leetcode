pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    nums.iter()
        .fold(
            Vec::<std::ops::RangeInclusive<i32>>::new(),
            |mut acc, num| {
                match acc.last_mut() {
                    None => acc.push(*num..=*num),
                    Some(range) => {
                        if !range.contains(num) {
                            // append in range
                            if *num == *range.end() + 1 {
                                *range = *range.start()..=*num;
                            }
                            // create new range
                            else {
                                acc.push(*num..=*num);
                            }
                        }
                    }
                };
                acc
            },
        )
        .iter()
        .map(|range| {
            if range.start() == range.end() {
                format!("{}", range.start())
            } else {
                format!("{}->{}", range.start(), range.end())
            }
        })
        .collect()
}

#[cfg(test)]
mod summary_ranges {

    #[test]
    fn example1() {
        let nums = vec![0, 1, 2, 4, 5, 7];
        let result = super::summary_ranges(nums);
        assert_eq!(result, vec!["0->2", "4->5", "7"]);
    }

    #[test]
    fn example2() {
        let nums = vec![0, 2, 3, 4, 6, 8, 9];
        let result = super::summary_ranges(nums);
        assert_eq!(result, vec!["0", "2->4", "6", "8->9"]);
    }
}
