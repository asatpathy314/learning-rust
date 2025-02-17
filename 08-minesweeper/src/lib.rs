pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut minefield_flattened: Vec<u8> = Vec::new();
    for row in minefield.iter() {
        for char in row.as_bytes() {
            minefield_flattened.push(*char)
        }
    }
    println!("{:?}", minefield_flattened);
    Vec::new()
}

pub fn main() {
    annotate(&[" * ", "   ", " * "]);
}
