fn main() {
    let b : [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for b in &b[2..5] {
        print!("{}, ", b);
    }
}
