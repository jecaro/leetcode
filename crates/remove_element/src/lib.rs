use std::vec::Vec;

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut not_val = 0;

    for i in (0..nums.len()).rev() {
        if nums[i] != val {
            not_val += 1;
        } else {
            nums.remove(i);
        }
    }

    not_val
}

#[cfg(test)]
mod remove_element {

    #[test]
    fn example1() {
        let mut nums = vec![3, 2, 2, 3];
        let result = super::remove_element(&mut nums, 3);
        assert_eq!(result, 2);
        assert_eq!(nums[..result as usize], vec![2, 2]);
    }

    #[test]
    fn example2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let result = super::remove_element(&mut nums, 2);
        assert_eq!(result, 5);
        assert_eq!(nums[..result as usize], vec![0, 1, 3, 0, 4]);
    }
}
