fn main() {
    let value : Option<&str> = Some("text");
    match value {
        Some(value) => println!("Wert ist: {}", value),
        None => println!("Kein Wert"),
    };
}
