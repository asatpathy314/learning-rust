pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut minefield_flattened: Vec<u8> = Vec::new();
    for row in minefield.iter() {
        for char in row.as_bytes() {
            minefield_flattened.push(*char)
        }
    }

    let row_size = minefield[0].len() as isize;
    let col_size = minefield.len() as isize;
    for ind in 0..minefield_flattened.len() as isize {
        if minefield_flattened[ind as usize] == 42 {  // the meaning of life...
            if ind - row_size >= 0 {
                minefield_flattened[(ind-row_size) as usize] += 1;
            }
            if ind - row_size - 1 >= 0 {
                minefield_flattened[(ind-row_size-1) as usize] += 1;
            }
            if ind - row_size + 1 >= 0 {
                minefield_flattened[(ind-row_size+1) as usize] += 1;
            }
            if ind - 1 >= 0 {
                minefield_flattened[(ind-1) as usize] += 1;
            }
            if ind + 1 < minefield_flattened.len() as isize {
                minefield_flattened[(ind+1) as usize] += 1;
            }
            if ind + row_size + 1 < minefield_flattened.len() as isize {
                minefield_flattened[(ind+row_size+1) as usize] += 1;
            }
            if ind + row_size - 1 < minefield_flattened.len() as isize {
                minefield_flattened[(ind+row_size-1) as usize] += 1;
            }
        }
    }
    for row_ind in 0..col_size {

    }
    Vec::new()
}

pub fn main() {
    annotate(&[" * ", "   ", " * "]);
}
