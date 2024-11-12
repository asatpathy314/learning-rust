use std::cmp::min;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let row_size = minefield[0].len();
    let result: Vec<Vec<u8>> = (0..minefield.len()).map(|_| Vec::with_capacity(row_size)).collect::<Vec<Vec<u8>>>();

    for row in minefield {
        let my_bytes: Vec<u8> = (*row).as_bytes().to_vec();
        for byte in my_bytes {
            if byte == b'*' {
                
            }
        }
    }

    return result;
}
