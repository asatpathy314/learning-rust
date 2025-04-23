pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let m: usize = minefield.len();
    let n: usize = if m > 0 { minefield[0].len() } else { 0 };

    let minefield: Vec<Vec<u8>> = minefield.into_iter()
        .map(|&row| {
            row.as_bytes().to_vec()
        })
        .collect();
    
    for (i: i32, row) in minefield.iter().enumerate() {
        for (j: i32, col) in row.iter().enumerate() {
            if *col == b'*' {

                let neighbors = get_neighbors(i, j, m, n);
            }
        }
    }
    


    vec![String::from("not implemented")]
}

fn get_neighbors(i: i32, j: i32, m: usize, n: usize) -> Vec<(i32, i32)> {
    let mut neighbors: Vec<(i32, i32)> = vec![]; 
    for x in -1..2 {
        for y in -1..2 {
            if is_in_bounds(i + x, j + y, m, n) && (x != 0 && y != 0) {
                neighbors.push((i + x, j + y));
            }
        }
    }
    neighbors
}

fn is_in_bounds(i: i32, j: i32, m: usize, n: usize) -> bool {
    i > -1 && i < m.try_into().unwrap() && j > -1 && j < n.try_into().unwrap()
}
