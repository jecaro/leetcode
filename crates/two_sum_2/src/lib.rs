pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[i] + numbers[j] == target {
                return vec![i, j].iter().map(|x| *x as i32 + 1).collect();
            }
        }
    }

    return Vec::new();
}

#[cfg(test)]
mod two_sum_2 {

    #[test]
    fn example1() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        let result = super::two_sum(numbers, target);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn example2() {
        let numbers = vec![2, 3, 4];
        let target = 6;
        let result = super::two_sum(numbers, target);
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn example3() {
        let numbers = vec![-1, 0];
        let target = -1;
        let result = super::two_sum(numbers, target);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn example4() {
        let numbers = vec![5, 25, 75];
        let target = 100;
        let result = super::two_sum(numbers, target);
        assert_eq!(result, vec![2, 3]);
    }
}
