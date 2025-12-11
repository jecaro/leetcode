use std::vec::Vec;

struct Current {
    value: i32,
    count: i32,
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut current = None;
    for i in (0..nums.len()).rev() {
        match current {
            None => {
                current = Some(Current {
                    value: nums[i],
                    count: 1,
                });
            }
            Some(ref mut curr) => {
                if nums[i] == curr.value {
                    curr.count += 1;
                    if curr.count > 2 {
                        nums.remove(i);
                    }
                } else {
                    current = Some(Current {
                        value: nums[i],
                        count: 1,
                    });
                }
            }
        }
    }
    nums.len() as i32
}

#[cfg(test)]
mod remove_duplicates_2 {

    #[test]
    fn example1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let k = super::remove_duplicates(&mut nums);
        assert_eq!(k, 5);
        assert_eq!(nums[..k as usize], [1, 1, 2, 2, 3]);
    }

    #[test]
    fn example2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let k = super::remove_duplicates(&mut nums);
        assert_eq!(k, 7);
        assert_eq!(nums[..k as usize], [0, 0, 1, 1, 2, 3, 3]);
    }
}
