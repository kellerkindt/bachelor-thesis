fn main() {
    let mut value : Option<u32> = Some(4);
    if let Some(ref mut value) = value {
        *value += 1;
    }
    println!("{:?}", value); // "Some(5)"
}
