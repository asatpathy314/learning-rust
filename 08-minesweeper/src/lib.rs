pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let size = minefield.len();
    let mut board: Vec<Vec<i32>> = vec![vec![0; size]; size];

    for (i, &str) in minefield.iter().enumerate() {
        let str_bytes = str.as_bytes();
        for (j, char) in str_bytes.iter().enumerate() {
            if *char == b'*' {
                board[i][j] = -1;
            }
        }
    }

    for (row_ind, row) in board.iter().enumerate() {
        for (col_ind, value) in row.iter().enumerate() {
            if *value == -1 {
                update_values(board, row_ind, col_ind);
            }
        }
    }

    Vec::with_capacity(10)
}

fn update_values(board: Vec<Vec<i32>>, row: usize, col: usize) -> () {
    
}
