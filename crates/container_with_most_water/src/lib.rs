pub fn area(height: &Vec<i32>, x1: usize, x2: usize) -> i32 {
    (x2 - x1) as i32 * std::cmp::min(height[x1], height[x2])
}

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut x1 = 0;
    let mut x2 = height.len() - 1;

    let mut max_area = (x2 - x1) as i32 * std::cmp::min(height[x1], height[x2]);
    loop {
        if height[x1] > height[x2] {
            x2 -= 1;
        } else {
            x1 += 1;
        }
        max_area = std::cmp::max(max_area, area(&height, x1, x2));
        if x1 == x2 {
            return max_area;
        }
    }
}

#[cfg(test)]
mod container_with_most_water {

    #[test]
    fn example1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = super::max_area(height);
        assert_eq!(result, 49);
    }

    #[test]
    fn example2() {
        let height = vec![1, 1];
        let result = super::max_area(height);
        assert_eq!(result, 1);
    }
}
