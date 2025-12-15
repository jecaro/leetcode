/// try to merge `first` into `interval`
///
/// # Arguments
/// - `from` the interval considered
/// - `to` the interval to merge in
///
/// # Returns
/// A boolean indicating if the interval has been merged or not
fn merge_interval(from: &Vec<i32>, to: &mut Vec<i32>) -> bool {
    let start1 = from[0];
    let end1 = from[1];
    let start2 = to[0];
    let end2 = to[1];

    let merged = !(start2 > end1 || start1 > end2);
    if merged {
        *to = vec![std::cmp::min(start1, start2), std::cmp::max(end1, end2)];
    }

    merged
}

pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut sorted: Vec<Vec<i32>> = intervals.into_iter().collect();
    sorted.sort_by_key(|x| x[0]);

    let mut result = Vec::new();
    for interval in sorted {
        match result.last_mut() {
            None => result.push(interval),
            Some(other_interval) => {
                if !merge_interval(&interval, other_interval) {
                    result.push(interval);
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod merge_intervals {

    #[test]
    fn example1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let result = super::merge(intervals);
        assert_eq!(result, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    }

    #[test]
    fn example2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let result = super::merge(intervals);
        assert_eq!(result, vec![vec![1, 5]]);
    }

    #[test]
    fn example3() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let result = super::merge(intervals);
        assert_eq!(result, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    }
}
