use std::vec::Vec;

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    for i in (1..nums.len()).rev() {
        if nums[i] == nums[i - 1] {
            nums.remove(i);
        }
    }
    nums.len() as i32
}

#[cfg(test)]
mod remove_duplicates {

    #[test]
    fn example1() {
        let mut nums = vec![1, 1, 2];
        let result = super::remove_duplicates(&mut nums);
        assert_eq!(result, 2);
        assert_eq!(nums[..result as usize], vec![1, 2]);
    }

    #[test]
    fn example2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let result = super::remove_duplicates(&mut nums);
        assert_eq!(result, 5);
        assert_eq!(nums[..result as usize], vec![0, 1, 2, 3, 4]);
    }
}
