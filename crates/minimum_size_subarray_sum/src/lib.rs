pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut x1 = 0;
    let mut sum = 0;
    let mut min_length = i32::max_value();

    for x2 in 0..nums.len() {
        sum += nums[x2];
        while sum >= target {
            min_length = std::cmp::min(min_length, x2 as i32 - x1 as i32 + 1);

            sum -= nums[x1];
            x1 += 1;
        }
    }

    if min_length == i32::max_value() {
        0
    } else {
        min_length
    }
}

#[cfg(test)]
mod minimum_size_subarray_sum {
    use crate::min_sub_array_len;

    #[test]
    fn example1() {
        let nums = vec![2, 3, 1, 2, 4, 3];
        let target = 7;
        let result = min_sub_array_len(target, nums);

        assert_eq!(result, 2);
    }

    #[test]
    fn example2() {
        let nums = vec![1, 4, 4];
        let target = 4;
        let result = min_sub_array_len(target, nums);

        assert_eq!(result, 1);
    }

    #[test]
    fn example3() {
        let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let target = 11;
        let result = min_sub_array_len(target, nums);

        assert_eq!(result, 0);
    }

    #[test]
    fn example4() {
        let nums = vec![12, 28, 83, 4, 25, 26, 25, 2, 25, 25, 25, 12];
        let target = 213;
        let result = min_sub_array_len(target, nums);

        assert_eq!(result, 8);
    }

    #[test]
    fn example5() {
        let nums = vec![1, 2, 3, 4, 5];
        let target = 15;
        let result = min_sub_array_len(target, nums);

        assert_eq!(result, 5);
    }

    #[test]
    fn example6() {
        let nums = vec![10, 2, 3];
        let target = 6;
        let result = min_sub_array_len(target, nums);

        assert_eq!(result, 1);
    }
}
