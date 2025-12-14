pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    for row in &board {
        if !check_chunk(row) {
            return false;
        }
    }

    for i in 0..9 {
        let col: Vec<_> = board.iter().map(|row| row[i]).collect();
        if !check_chunk(&col) {
            return false;
        }
    }

    for cell_x in 0..3 {
        for cell_y in 0..3 {
            let mut cell = Vec::new();
            for i in 0..3 {
                let slice = &board[cell_y * 3 + i][cell_x * 3..cell_x * 3 + 3];
                cell.append(&mut slice.to_vec());
            }

            if !check_chunk(&cell) {
                return false;
            }
        }
    }
    true
}

pub fn check_chunk(chunk: &Vec<char>) -> bool {
    let mut sorted = chunk.clone();
    sorted.sort();
    sorted
        .iter()
        .fold((true, Vec::<char>::new()), |acc, char| match acc {
            (false, seen) => (false, seen),
            (true, mut seen) => match *char {
                '.' => (true, seen),
                '0'..='9' => {
                    // println!("char {:?}", char);
                    if seen.contains(char) {
                        // println!("seen");
                        (false, seen)
                    } else {
                        // println!("not seen");
                        seen.push(*char);
                        (true, seen)
                    }
                }
                _ => (false, seen),
            },
        })
        .0
}

#[cfg(test)]
mod valid_sudoku {

    #[test]
    fn example1() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let result = super::is_valid_sudoku(board);
        assert_eq!(result, true);
    }

    #[test]
    fn example2() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let result = super::is_valid_sudoku(board);
        assert_eq!(result, false);
    }
}
