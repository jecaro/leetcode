#[derive(Debug, Clone, Copy)]
enum Side {
    Right,
    Down,
    Left,
    Up,
}

const SIDES: [Side; 4] = [Side::Right, Side::Down, Side::Left, Side::Up];

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.len() == 0 {
        return vec![];
    }

    let mut x1: i32 = 0;
    let mut x2: i32 = matrix[0].len() as i32 - 1;
    let mut y1: i32 = 0;
    let mut y2: i32 = matrix.len() as i32 - 1;

    let mut result: Vec<i32> = Vec::new();
    let mut sides = SIDES.iter();
    // With a single column, we start down
    if matrix[0].len() == 1 {
        sides.next();
    }

    for side in sides.cycle() {
        match side {
            Side::Right => {
                for x in x1..=x2 {
                    result.push(matrix[y1 as usize][x as usize]);
                }
                y1 += 1;
            }
            Side::Down => {
                for y in y1..=y2 {
                    result.push(matrix[y as usize][x2 as usize]);
                }
                x2 -= 1;
            }
            Side::Left => {
                for x in (x1..=x2).rev() {
                    result.push(matrix[y2 as usize][x as usize]);
                }
                if y2 != 0 {
                    y2 -= 1;
                }
            }
            Side::Up => {
                for y in (y1..=y2).rev() {
                    result.push(matrix[y as usize][x1 as usize]);
                }
                x1 += 1;
            }
        }

        if x1 > x2 || y1 > y2 {
            break;
        }
    }

    result
}

#[cfg(test)]
mod spiral_matrix {

    #[test]
    fn example1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = super::spiral_order(matrix);
        assert_eq!(result, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn example2() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let result = super::spiral_order(matrix);
        assert_eq!(result, vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
    }

    #[test]
    fn example3() {
        let matrix = vec![vec![3], vec![2]];
        let result = super::spiral_order(matrix);
        assert_eq!(result, vec![3, 2]);
    }
}
