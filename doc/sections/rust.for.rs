fn main() {
    let array = [1, 2, 3, 4, 5, 6];

    for i in 0..array.len() {
        println!("Index: {}, Wert: {}", i, array[i]);
    }

    for a in &array {
        println!("Wert: {}", a);
    }
}
