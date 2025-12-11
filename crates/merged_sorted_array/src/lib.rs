use std::vec::Vec;

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, _n: i32) {
    let mut nums1_i = 0;
    let mut nums1_size = m as usize;

    for num in nums2.iter() {
        while *num > nums1[nums1_i] && nums1_i < nums1_size {
            nums1_i += 1;
        }

        nums1.insert(nums1_i, *num);
        nums1.pop();
        nums1_size += 1;
    }
}

#[cfg(test)]
mod merged_sorted_array {

    #[test]
    fn example1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];

        super::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn example2() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];

        super::merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn example3() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];

        super::merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(nums1, vec![1]);
    }
}
